use std::fs;
use anyhow::{Context, Result};
use clap::Parser;
use log::info;

#[derive(Parser)]
struct Args {
    #[clap(default_value = "./etherscan-api-key.txt", long, short = 'k')]
    api_key_file_path: String,
    #[clap(default_value = "https://api.etherscan.io", long, short = 'u')]
    api_url: String,
    #[clap(long, short = 'a')]
    contract_address: String,
}

fn main() -> Result<()> {
    info!("start up");

    let args = Args::parse();

    let api_key = fs::read_to_string(&args.api_key_file_path).context("failed to read API key file")?;

    esctl::download_source_code_files(
        api_key,
        args.api_url,
        args.contract_address,
    )
}
