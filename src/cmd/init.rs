use clap::{Parser};
use anyhow::{Result};

use crate::hetzner::hetzner::HetznerProvider;

#[derive(Parser)]
#[clap(name = "init", about = "Init account")]
pub struct Init {}

impl Init {
    pub async fn execute(&self) -> Result<()> {
        let hetzner = HetznerProvider::new_provider(true);
        match hetzner {
            Ok(provider) => provider.init().await,
            Err(err) => return Err(err),
        }.expect("TODO: panic message");
        Ok(())
    }
}
