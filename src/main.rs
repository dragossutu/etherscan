use anyhow::{Context, Error, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use log::{info};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const ETHERSCAN_API_KEY_FILE_PATH: &str = "./etherscan-api-key.txt";
const CONTRACTS_DEST_DIR: &str = "./contracts";

#[derive(Parser, Debug)]
struct Args {
    contract_address: String,
    #[clap(short, long)]
    api_key_file_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GetContractRes {
    result: Vec<GetContractResult>,
}

#[derive(Serialize, Deserialize)]
struct GetContractResult {
    #[serde(alias = "ABI")]
    abi: String,
    #[serde(alias = "ContractName")]
    contract_name: String,
    #[serde(alias = "SourceCode")]
    source_code: String,
}

#[derive(Serialize, Deserialize)]
struct GetContractSourceCode {
    sources: HashMap<String, GetContractContent>,
}

#[derive(Serialize, Deserialize)]
struct GetContractContent {
    content: String,
}

fn main() -> Result<()> {
    info!("start up");

    let args = Args::parse();

    // use default value for api_key_file_path if no value was passed to CLI
    let api_key_file_path = match &args.api_key_file_path {
        Some(_path) => _path,
        None => ETHERSCAN_API_KEY_FILE_PATH,
    };

    let api_key = std::fs::read_to_string(api_key_file_path)
        .with_context(|| format!("could not read file at path: `{:?}`", api_key_file_path))?;

    let url = format!(
        "https://api.etherscan.io/api?module=contract&action=getsourcecode&address={}&apikey={}",
        &args.contract_address,
        api_key,
    );

    let res = reqwest::blocking::get(&url)?;
    if !res.status().is_success() {
        return Result::Err(
            Error::msg(format!("request failed, http code: {}", res.status().as_str()))
        );
    }

    let body = res.text()?;

    let body_json: GetContractRes = serde_json::from_str(&body)?;

    for result in body_json.result.iter() {
        let mut r = result.source_code.strip_prefix("{")
            .ok_or(Error::msg("failed to remove `{` prefix from source_code"))?;

        r = r.strip_suffix("}")
            .ok_or(Error::msg("failed to remove `}` suffix from source_code"))?;

        let source_codes: GetContractSourceCode = serde_json::from_str(r)?;

        for (contract_path, contract_code) in source_codes.sources {
            let mut p = Path::new(&contract_path);

            // some contracts path contain a leading "/", make sure it's removed
            if p.starts_with("/") {
                p = p.strip_prefix("/")?;
            }

            // make sure dirs in path exist
            let contract_dir = Path::new(CONTRACTS_DEST_DIR).join(p.parent().unwrap());
            fs::create_dir_all(&contract_dir)?;

            // create solidity file
            fs::write(contract_dir.join(p.file_name().unwrap()), contract_code.content);
        }
    }

    Ok(())
}
