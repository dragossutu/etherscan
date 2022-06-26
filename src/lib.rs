use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

mod contracts;
use contracts::client::Request;
use contracts::service::Contracts;

pub const CONTRACTS_DEST_DIR: &str = "./contracts";

pub fn go(
    api_key: String,
    api_url: Option<String>,
    contract_address: String,
    files_dest_path: String,
    network: &str,
) -> Result<()> {
    let http_client = reqwest::blocking::Client::new();

    let contracts_client = contracts::client::Client::new(api_key, api_url, http_client, network)
        .context("failed to create HTTP client")?;

    let contracts_service = contracts::service::Service::new(&contracts_client);

    let contracts = contracts_service
        .get_contracts(&contract_address)
        .context("contracts_service failed to get contracts")?;

    for c in contracts.iter() {
        let mut p = Path::new(&c.path);

        // some contracts path contain a leading "/",
        // make sure it's removed before creating the file
        if p.starts_with("/") {
            p = p.strip_prefix("/").unwrap();
        }

        // make sure the file has '.sol' extension
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

        // create all the dirs in Solidity file's path
        if p.parent().is_none() {
            return Err(anyhow!(
                "failed to get path without file name from path: {:?}",
                p
            ));
        }

        let contract_dir = Path::new(&files_dest_path)
            .join(&network)
            .join(&contract_address)
            .join(p.parent().unwrap());
        fs::create_dir_all(&contract_dir).context("failed to create contracts dir")?;

        // create Solidity file
        let contract_file_path = contract_dir.join(file_name);
        fs::write(contract_file_path, &c.code)
            .context("failed to crate and write to solidity file")?;
    }

    Ok(())
}
