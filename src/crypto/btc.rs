use anyhow::Result;
use walletd::walletd_bip39::prelude::*;    
use walletd::walletd_bitcoin::AddressType;
use walletd::walletd_hd_key::prelude::*;
use walletd::walletd_bitcoin::prelude::*;
use crate::WalletInfo;

pub fn generate(addr_type: AddressType) -> Result<WalletInfo> {
    let mnemonic = Bip39Mnemonic::builder()
        .mnemonic_type(Bip39MnemonicType::Words12)
        .build()?;
    let seed = mnemonic.to_seed();

    let master_hd_key = HDKey::new_master(seed.clone(), HDNetworkType::MainNet)?;

    let wallet = BitcoinWallet::builder()
        .master_hd_key(master_hd_key.clone())
        .address_format(addr_type)
        .build()?;

    let wallet_addr = wallet.receive_address()?;
    let wif: String = master_hd_key.to_wif()?;

    let result = WalletInfo {
        wallet_address: wallet_addr,
        private_key: wif
    };

    Ok(result)
}