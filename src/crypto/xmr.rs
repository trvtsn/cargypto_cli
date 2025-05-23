use anyhow::{Result};
use monero_address_creator::{Seed, network::Mainnet, error::Error};
use crate::WalletInfo;

pub fn generate() -> Result<WalletInfo, Error> {
    match Seed::generate() {
        Ok(seed) => {
            let wallet_addr = seed.to_address::<Mainnet>().unwrap();
            let seed_words = seed.seed_words().unwrap().join(" ");

            let result = WalletInfo {
                wallet_address: wallet_addr,
                private_key: seed_words
            };

            Ok(result)
        }
        Err(e) => {
            Err(e)
        }
    }
}