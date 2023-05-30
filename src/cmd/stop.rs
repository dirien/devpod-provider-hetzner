use clap::{Parser};
use anyhow::{Result};
use crate::hetzner::hetzner::HetznerProvider;

#[derive(Parser)]
#[clap(name = "stop", about = "Stop an instance")]
pub struct Stop {}

impl Stop {
    pub async fn execute(&self) -> Result<()> {
        let hetzner = HetznerProvider::new_provider(false);
        match hetzner {
            Ok(provider) => {
                let stop = provider.stop().await;
                match stop {
                    Err(err) => return Err(anyhow::anyhow!("Error stopping instance: {}", err)),
                    _ => {}
                }
            }
            Err(err) => return Err(err),
        };
        Ok(())
    }
}
