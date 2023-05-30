use clap::{Parser};
use anyhow::{Result};
use crate::hetzner::hetzner::HetznerProvider;

#[derive(Parser)]
#[clap(name = "status", about = "Status an instance")]
pub struct Status {}

impl Status {
    pub async fn execute(&self) -> Result<()> {
        let hetzner = HetznerProvider::new_provider(false);
        match hetzner {
            Ok(provider) => {
                let status = provider.status().await;
                match status {
                    Err(err) => return Err(anyhow::anyhow!("Error getting instance status: {}", err)),
                    _ => {}
                }
                println!("{}", status.unwrap());
            }
            Err(err) => return Err(err),
        }
        Ok(())
    }
}
