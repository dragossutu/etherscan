use clap::Parser;
use log::info;
use std::error::Error;
use std::fs;
use std::path::Path;

mod contracts;

use contracts::client::Request;
use contracts::service::Contracts;

const ETHERSCAN_API_KEY_FILE_PATH: &str = "./etherscan-api-key.txt";
const CONTRACTS_DEST_DIR: &str = "./contracts";

#[derive(Parser)]
struct Args {
    contract_address: String,
    #[clap(short, long)]
    api_key_file_path: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    info!("start up");

    let args = Args::parse();

    // use default value for api_key_file_path if no value was passed to CLI
    let api_key_file_path = match &args.api_key_file_path {
        Some(_path) => _path,
        None => ETHERSCAN_API_KEY_FILE_PATH,
    };

    let api_key = std::fs::read_to_string(api_key_file_path)?;

    let http_client = reqwest::blocking::Client::new();
    let contracts_client = contracts::client::Client::new(http_client, api_key);
    let contracts_service = contracts::service::Service::new(&contracts_client);

    let contracts = contracts_service.get_contracts(&args.contract_address)?;

    for c in contracts.iter() {
        let mut p = Path::new(&c.path);

        // some contracts path contain a leading "/", make sure it's removed
        if p.starts_with("/") {
            p = p.strip_prefix("/")?;
        }

        // make sure all dirs in path exist
        let contract_dir = Path::new(CONTRACTS_DEST_DIR).join(p.parent().unwrap());
        fs::create_dir_all(&contract_dir)?;

        // create solidity file
        if p.file_name().is_none() {
            return Err(From::from("failed to get file name"));
        }

        let contract_file_path = contract_dir.join(p.file_name().unwrap());
        fs::write(contract_file_path, &c.code)?;
    }

    Ok(())
}
