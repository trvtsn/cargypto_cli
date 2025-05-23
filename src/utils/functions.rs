use crate::WalletInfo;
use crate::OutputFormat;

pub fn get_wallet(wallet_info: WalletInfo, output_format: OutputFormat) -> String {
    let wallet_address = wallet_info.wallet_address;
    let private_key = wallet_info.private_key;
    let result: String;

    match output_format {
        OutputFormat::CSV => {
            result = format!(
                "\"{}\",\"{}\"", 
                wallet_address, 
                private_key
            );
        }
    }

    return result;
}