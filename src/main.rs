use anyhow::Result;
use clap::Parser;
use log::info;

#[derive(Parser)]
struct Args {
    #[clap(env = "SCAN_API_KEY", long, short = 'k')]
    api_key: String,
    #[clap(default_value = "https://api.etherscan.io", long, short = 'u')]
    api_url: String,
    contract_address: String,
    #[clap(default_value = "./contracts", long, short = 'd')]
    files_dest_path: String,
}

fn main() -> Result<()> {
    info!("start up");

    let args = Args::parse();

    scancli::go(
        args.api_key,
        args.api_url,
        args.contract_address,
        args.files_dest_path,
    )
}
