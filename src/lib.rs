#![allow(non_snake_case)]

mod crypto;
mod utils;

use anyhow::Error;
use clap::ValueEnum;
use crypto::btc;
use crypto::ltc;
use crypto::eth;
use crypto::xmr;
use walletd::walletd_bitcoin::AddressType as AddressTypeWalletd;

#[derive(Debug)]
pub struct Crypto;

#[derive(Debug)]
pub struct Utils;

#[derive(Debug)]
pub struct WalletInfo {
    pub wallet_address: String,
    pub private_key: String,
}

#[derive(Debug, Clone, Copy)]
pub enum AddressType {
    P2PKH,
    P2SH,
    Bech32,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lower")]
pub enum OutputFormat {
    CSV
}

impl From<AddressType> for AddressTypeWalletd {
    fn from(a: AddressType) -> AddressTypeWalletd {
        match a {
            AddressType::P2PKH  => AddressTypeWalletd::P2pkh,
            AddressType::P2SH   => AddressTypeWalletd::P2sh,
            AddressType::Bech32 => AddressTypeWalletd::P2wpkh,
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OutputFormat::CSV => "csv",
            // OutputFormat::JSON => "json",
            // ...
        };
        write!(f, "{s}")
    }
}

impl Crypto {
    pub fn btc(addr_type: AddressType) -> Result<WalletInfo, Error> {
        return btc::generate(addr_type.into());
    } 

    pub fn ltc() -> Result<WalletInfo, Error> {
        return ltc::generate();
    }

    pub fn eth() -> Result<WalletInfo, Error> {
        return eth::generate();
    }

    pub fn xmr() -> Result<WalletInfo, monero_address_creator::error::Error> {
        return xmr::generate();
    }
}

impl Utils {
    pub fn get_wallet(wallet_info: WalletInfo, output_format: OutputFormat) -> String {
        return utils::functions::get_wallet(wallet_info, output_format);
    } 
}