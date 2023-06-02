use clap::{Parser};
use anyhow::{Result};

use crate::hetzner::hetzner::HetznerProvider;

#[derive(Parser)]
#[clap(name = "start", about = "Start an instance")]
pub struct Start {}

impl Start {
    pub async fn execute(&self) -> Result<()> {
        let hetzner = HetznerProvider::new_provider(false);
        match hetzner {
            Ok(provider) => {
                let start = provider.start().await;
                match start {
                    Err(err) => return Err(anyhow::anyhow!("Error starting instance: {}", err)),
                    _ => {}
                }
            }
            Err(err) => return Err(err),
        };
        Ok(())
    }
}
