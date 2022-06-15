use clap::Parser;
use log::info;
use std::error::Error;

#[derive(Parser)]
struct Args {
    contract_address: String,
    #[clap(short, long)]
    api_key_file_path: Option<String>,
}

const ETHERSCAN_API_KEY_FILE_PATH: &str = "./etherscan-api-key.txt";

fn main() -> Result<(), Box<dyn Error>> {
    info!("start up");

    let args = Args::parse();

    // use default value for api_key_file_path if no value was passed to CLI
    let api_key_file_path = match &args.api_key_file_path {
        Some(_path) => _path,
        None => ETHERSCAN_API_KEY_FILE_PATH,
    };

    let api_key = std::fs::read_to_string(api_key_file_path)?;

    esctl::just_do_it(api_key, args.contract_address)
}
