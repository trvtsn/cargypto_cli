use anyhow::Result;
use walletd::walletd_bip39::prelude::*;
use walletd::walletd_hd_key::prelude::*;
use walletd::walletd_ethereum::prelude::*;
use walletd::walletd_mnemonics_core::hex;
use crate::WalletInfo;

pub fn generate() -> Result<WalletInfo> {
    let mnemonic = Bip39Mnemonic::builder()
        .mnemonic_type(Bip39MnemonicType::Words12)
        .build()?;
    let seed = mnemonic.to_seed();

    let master_hd_key = HDKey::new_master(seed.clone(), HDNetworkType::MainNet)?;

    let wallet = EthereumWallet::builder()
        .master_hd_key(master_hd_key.clone())
        .build()?;

    let wallet_addr = wallet.public_address();
    let private_key = hex::encode(wallet.private_key()?.to_bytes());

    let result = WalletInfo {
        wallet_address: wallet_addr,
        private_key: private_key
    };

    Ok(result)
}