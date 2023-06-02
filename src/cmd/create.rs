use clap::{Parser};
use anyhow::{Result};

use crate::hetzner::hetzner::HetznerProvider;

#[derive(Parser)]
#[clap(name = "create", about = "Create an instance")]
pub struct Create {}

impl Create {
    pub async fn execute(&self) -> Result<()> {
        let hetzner = HetznerProvider::new_provider(false);
        match hetzner {
            Ok(provider) => {
                let create = provider.create().await;
                match create {
                    Err(err) => return Err(anyhow::anyhow!("Error creating instance: {}", err)),
                    _ => {}
                }
            }
            Err(err) => return Err(err),
        }
        Ok(())
    }
}
