use anyhow::{Error, anyhow};
use clap::{Parser, Subcommand};
use cargypto_cli::{Crypto, AddressType, Utils, OutputFormat};

#[derive(Parser)]
#[command(name = "cargypto_cli")]
struct Args {
    #[command(subcommand)]
    coin: Coin,

    #[arg(short, long, default_value_t = 1)]
    count: i32,

    #[arg(short, long, default_value_t = OutputFormat::CSV)]
    output_format: OutputFormat,
}

#[derive(Subcommand)]
enum Coin {
    BTC {
        #[arg(value_parser = ["p2pkh","p2sh","bech32"])]
        addr_type: String,
    },
    LTC,
    ETH,
    XMR,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let output_format = args.output_format;

    let mut i = 0;
    while i < args.count {
        match &args.coin {
            Coin::BTC { addr_type } => {
                let at = match addr_type.as_str() {
                    "p2pkh"  => AddressType::P2PKH,
                    "p2sh"   => AddressType::P2SH,
                    "bech32" => AddressType::Bech32,
                    other    => return Err(anyhow!("Invalid address type: {}", other)),
                };

                match Crypto::btc(at) {
                    Ok(wallet_info) => {
                        i += 1;

                        let result = Utils::get_wallet(wallet_info, output_format);
                        println!("\"{}\",{}", i, result);
                    },
                    Err(_) => {
                        i -= 1;

                        continue;
                    }
                }
            }

            Coin::LTC => {
                match Crypto::ltc() {
                    Ok(wallet_info) => {
                        i += 1;
                        
                        let result = Utils::get_wallet(wallet_info, output_format);
                        println!("\"{}\",{}", i, result);
                    },
                    Err(_) => {
                        i -= 1;

                        continue;
                    }
                }
            }

            Coin::ETH => {
                match Crypto::eth() {
                    Ok(wallet_info) => {
                        i += 1;
                        
                        let result = Utils::get_wallet(wallet_info, output_format);
                        println!("\"{}\",{}", i, result);
                    },
                    Err(_) => {
                        i -= 1;

                        continue;
                    }
                }
            }

            Coin::XMR => {
                match Crypto::xmr() {
                    Ok(wallet_info) => {
                        i += 1;
                        
                        let result = Utils::get_wallet(wallet_info, output_format);
                        println!("\"{}\",{}", i, result);
                    },
                    Err(_) => {
                        i -= 1;
                        
                        continue;
                    }
                }
            }
        }
    }

    Ok(())
}