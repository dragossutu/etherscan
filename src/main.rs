use anyhow::Result;
use clap::Parser;
use log::info;

#[derive(Parser)]
struct Args {
    contract_address: String,
    #[clap(short, long)]
    api_key_file_path: Option<String>,
}

const ETHERSCAN_API_KEY_FILE_PATH: &str = "./etherscan-api-key.txt";
const ETHERSCAN_API_URL: &str = "https://api.etherscan.io";

fn main() -> Result<()> {
    info!("start up");

    let args = Args::parse();

    // use default value for api_key_file_path if no value was passed to CLI
    let api_key_file_path = match &args.api_key_file_path {
        Some(_path) => _path,
        None => ETHERSCAN_API_KEY_FILE_PATH,
    };

    let api_key = std::fs::read_to_string(api_key_file_path)?;

    esctl::download_source_code_files(
        api_key,
        ETHERSCAN_API_URL.to_string(),
        args.contract_address,
    )
}
