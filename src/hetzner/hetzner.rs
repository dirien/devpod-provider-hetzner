use std::collections::HashMap;
use hcloud::apis::configuration::Configuration;
use anyhow::{Context, Result};
use std::env;
use hcloud::models::Server;
use crate::options::options::{from_env, Options};
use crate::ssh::keys;

pub struct HetznerProvider {
    configuration: Configuration,
    pub options: Options,
}

impl HetznerProvider {
    pub fn new_provider(init: bool) -> Result<HetznerProvider> {
        let token = env::var("HCLOUD_TOKEN").context("Please set HCLOUD_TOKEN environment variable");
        match token {
            Ok(token) => {
                let mut configuration = Configuration::new();
                configuration.bearer_access_token = Some(token);

                let options = from_env(init);
                let provider = HetznerProvider {
                    configuration,
                    options,
                };
                Ok(provider)
            }
            Err(err) => return Err(err),
        }
    }

    pub async fn get_devpod_instance(&self) -> Result<Box<Server>> {
        let servers = hcloud::apis::servers_api::list_servers(&self.configuration, hcloud::apis::servers_api::ListServersParams {
            label_selector: Some(format!("{}=true", self.options.machine_id)),
            ..Default::default()
        }).await?.servers;

        if servers.len() == 0 {
            return Err(anyhow::anyhow!("No devpod instance found"));
        }
        let server_id = servers.get(0).unwrap().id.clone();


        let server = hcloud::apis::servers_api::get_server(&self.configuration, hcloud::apis::servers_api::GetServerParams {
            id: server_id,
        }).await?.server.unwrap();
        Ok(server)
    }
    pub async fn init(&self) -> Result<()> {
        let _list = hcloud::apis::servers_api::list_servers(&self.configuration, hcloud::apis::servers_api::ListServersParams::default()).await?;
        Ok(())
    }

    pub async fn delete(&self) -> Result<()> {
        let devpod_instance = self.get_devpod_instance().await?;
        hcloud::apis::servers_api::delete_server(&self.configuration, hcloud::apis::servers_api::DeleteServerParams {
            id: devpod_instance.id,
        }).await?;
        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        let devpod_instance = self.get_devpod_instance().await?;
        hcloud::apis::servers_api::power_on_server(&self.configuration, hcloud::apis::servers_api::PowerOnServerParams {
            id: devpod_instance.id,
        }).await?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let devpod_instance = self.get_devpod_instance().await?;
        hcloud::apis::servers_api::power_off_server(&self.configuration, hcloud::apis::servers_api::PowerOffServerParams {
            id: devpod_instance.id,
        }).await?;
        Ok(())
    }

    pub async fn status(&self) -> Result<String> {
        let devpod_instance = self.get_devpod_instance().await?;
        return if devpod_instance.status == hcloud::models::server::Status::Running {
            Ok("Running".to_string())
        } else if devpod_instance.status == hcloud::models::server::Status::Off {
            Ok("Stopped".to_string())
        } else {
            Ok("Busy".to_string())
        };
    }

    pub async fn create(&self) -> Result<()> {
        let public_key_base = keys::get_public_key_base(self.options.machine_folder.clone());

        let list_image_params = hcloud::apis::images_api::ListImagesParams {
            name: Some(self.options.image.clone()),
            architecture: Some(self.options.architecture.clone()),
            ..Default::default()
        };

        let images = hcloud::apis::images_api::list_images(&self.configuration, list_image_params).await;
        match images {
            Err(err) => return Err(anyhow::anyhow!("Error getting image list: {}", err)),
            _ => {}
        }
        let image_list = images.unwrap();
        if image_list.clone().images.len() == 0 {
            return Err(anyhow::anyhow!("No image found"));
        }
        let image_id = image_list.clone().images.get(0).unwrap().id.clone();

        let locations = hcloud::apis::locations_api::list_locations(&self.configuration, hcloud::apis::locations_api::ListLocationsParams {
            name: Some(self.options.location.clone()),
            ..Default::default()
        }).await;
        match locations {
            Err(err) => return Err(anyhow::anyhow!("Error getting location list: {}", err)),
            _ => {}
        }
        let location_list = locations.unwrap();
        if location_list.clone().locations.len() == 0 {
            return Err(anyhow::anyhow!("No location found"));
        }
        let location_id = location_list.clone().locations.get(0).unwrap().id.clone();

        let server_types = hcloud::apis::server_types_api::list_server_types(&self.configuration, hcloud::apis::server_types_api::ListServerTypesParams {
            name: Some(self.options.server_type.clone()),
            ..Default::default()
        }).await;
        match server_types {
            Err(err) => return Err(anyhow::anyhow!("Error getting server type list: {}", err)),
            _ => {}
        }
        let server_type_list = server_types.unwrap();
        if server_type_list.clone().server_types.len() == 0 {
            return Err(anyhow::anyhow!("No server type found"));
        }
        let server_type_name = server_type_list.clone().server_types.get(0).unwrap().name.clone();

        let mut server_params = hcloud::apis::servers_api::CreateServerParams::default();

        let mut labels = HashMap::new();
        labels.insert(self.options.machine_id.clone(), "true".to_string());

        server_params.create_server_request = Some(hcloud::models::CreateServerRequest {
            name: petname::petname(3, "-"),
            server_type: server_type_name,
            image: image_id.to_string(),
            location: Some(location_id.to_string()),
            labels: Some(labels),
            user_data: Some(format!(r#"#cloud-config
users:
- name: devpod
  shell: /bin/bash
  groups: [ sudo, docker ]
  ssh_authorized_keys:
  - {}
  sudo: [ "ALL=(ALL) NOPASSWD:ALL" ]"#, public_key_base)),
            ..Default::default()
        });
        let mut server = hcloud::apis::servers_api::create_server(&self.configuration, server_params).await?.server;

        let mut still_creating = true;

        while still_creating {
            server = hcloud::apis::servers_api::get_server(&self.configuration, hcloud::apis::servers_api::GetServerParams {
                id: server.id.clone(),
            }).await?.server.unwrap();
            if server.status == hcloud::models::server::Status::Running {
                still_creating = false;
            } else {
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }
        Ok(())
    }
}


