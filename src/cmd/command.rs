use std::env;
use clap::{Parser};
use anyhow::{Result};
use crate::hetzner::hetzner::HetznerProvider;
use crate::ssh;

#[derive(Parser)]
#[clap(name = "command", about = "Command an instance")]
pub struct Command {}

impl Command {
    pub async fn execute(&self) -> Result<()> {
        let hetzner = HetznerProvider::new_provider(false);
        match hetzner {
            Ok(provider) => {
                let command = env::var("COMMAND");
                let private_key;
                #[cfg(any(target_os = "linux", target_os = "macos"))]
                {
                    private_key = ssh::keys::get_private_key_raw_base(provider.options.machine_folder.clone());
                }
                #[cfg(target_os = "windows")]
                {
                    private_key = ssh::keys::get_private_key_filename(provider.options.machine_folder.clone());
                }
                let instance = provider.get_devpod_instance().await;
                match instance {
                    Err(err) => return Err(err),
                    _ => {}
                }

                let client = ssh::helper::new_ssh_client("devpod".to_string(), instance.unwrap().public_net.ipv4.unwrap().ip.clone(),
                                                         private_key.clone());
                match client {
                    Err(err) => return Err(anyhow::anyhow!("Error creating ssh client: {}", err)),
                    _ => {}
                }
                let result = ssh::helper::execute_command(command.unwrap(), client.unwrap());
                match result {
                    Err(err) => return Err(anyhow::anyhow!("Error executing command: {}", err)),
                    _ => {
                        println!("{}", result.unwrap());
                    }
                }
            }
            Err(err) => return Err(err),
        };
        Ok(())
    }
}
