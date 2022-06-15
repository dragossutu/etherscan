use std::error::Error;
use std::fs;
use std::path::Path;

mod contracts;
use contracts::client::Request;
use contracts::service::Contracts;

const CONTRACTS_DEST_DIR: &str = "./contracts";

pub fn just_do_it(api_key: String, contract_address: String) -> Result<(), Box<dyn Error>> {
    let http_client = reqwest::blocking::Client::new();

    let contracts_client = contracts::client::Client::new(http_client, api_key);

    let contracts_service = contracts::service::Service::new(&contracts_client);

    let contracts = contracts_service.get_contracts(&contract_address)?;

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
