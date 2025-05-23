use anyhow::Result;
use secp256k1::Secp256k1;
use crypto_addr::{WIF, Address, WIFFormat, AddressFormat};
use crate::WalletInfo;

pub fn generate() -> Result<WalletInfo> {
    let secp = Secp256k1::new();
    let mut rng = rand::rng();

    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    let wif = WIF::wif(
        &secret_key[..], 
        WIFFormat::Litecoin
    )?;

    let wallet_addr = Address::addr(
        &public_key.serialize(), 
        &AddressFormat::Litecoin
    )?;

    let result = WalletInfo {
        wallet_address: wallet_addr,
        private_key: wif
    };

    Ok(result)
}