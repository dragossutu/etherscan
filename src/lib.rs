use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

mod contracts;
use contracts::client::Request;
use contracts::service::Contracts;

pub const CONTRACTS_DEST_DIR: &str = "./contracts";

pub fn download_source_code_files(
    api_key: String,
    api_url: String,
    contract_address: String,
) -> Result<()> {
    let http_client = reqwest::blocking::Client::new();

    let contracts_client = contracts::client::Client::new(api_key, api_url, http_client);

    let contracts_service = contracts::service::Service::new(&contracts_client);

    let contracts = contracts_service.get_contracts(&contract_address)?;

    for c in contracts.iter() {
        let mut p = Path::new(&c.path);

        // some contracts path contain a leading "/", make sure it's removed
        if p.starts_with("/") {
            p = p.strip_prefix("/")?;
        }

        if p.file_name().is_none() {
            return Err(anyhow!("failed to get file name from path: {:?}", p));
        }

        let mut file_name = p.file_name().unwrap().to_str().unwrap();
        let mut tmp: String;
        if p.extension().is_none() {
            tmp = String::from(file_name);
            tmp.push_str(".sol");
            file_name = tmp.as_str();
        }

        // create dirs in Solidity file's path
        if p.parent().is_none() {
            return Err(anyhow!(
                "failed to get path without file name from path: {:?}",
                p
            ));
        }

        let contract_dir = Path::new(CONTRACTS_DEST_DIR)
            .join(&contract_address)
            .join(p.parent().unwrap());
        fs::create_dir_all(&contract_dir)?;

        // create Solidity file
        let contract_file_path = contract_dir.join(file_name);
        fs::write(contract_file_path, &c.code)?;
    }

    Ok(())
}
