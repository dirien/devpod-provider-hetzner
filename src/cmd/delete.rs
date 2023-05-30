use clap::{Parser};
use anyhow::{Result};


use crate::hetzner::hetzner::HetznerProvider;

#[derive(Parser)]
#[clap(name = "delete", about = "Delete an instance")]
pub struct Delete {}

impl Delete {
    pub async fn execute(&self) -> Result<()> {
        let hetzner = HetznerProvider::new_provider(false);
        match hetzner {
            Ok(provider) => {
                let delete = provider.delete().await;
                match delete {
                    Err(err) => return Err(anyhow::anyhow!("Error deleting instance: {}", err)),
                    _ => {}
                }
            }
            Err(err) => return Err(err),
        };
        Ok(())
    }
}
