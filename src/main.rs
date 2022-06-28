use anyhow::{anyhow, Context, Result};
use clap::Parser;
use log::info;
use reqwest::blocking::Client as ReqwestClient;
use std::fs;
use std::path::Path;

mod contracts;
use contracts::client::{Client, Request};
use contracts::service::{Contracts, Service};

pub const CONTRACTS_DEST_DIR: &str = "./contracts";

#[derive(Parser)]
#[clap(about, version)]
struct Args {
    #[clap(env = "SCAN_API_KEY", long, short = 'k')]
    /// The API key for the block explorer.
    /// It will be read from the environment variable SCAN_API_KEY first
    api_key: String,
    #[clap(long, short = 'u')]
    /// The URL of the block explorer's API, e.g. https://api.etherscan.io. Used for tests only.
    /// If passed in, the `network` argument is ignored
    api_url: Option<String>,
    /// Address of the contract to download files for
    contract_address: String,
    #[clap(default_value = "./contracts", long, short = 'd')]
    /// Local path to the folder where the contract's files will be created.
    /// Folder will be created if it doesn't exist
    files_dest_path: String,
    #[clap(default_value_t = Network::Ethereum, long, value_enum)]
    /// The name of the network.
    /// Must match the block explorer that the API key is for. e.g arbitrum will make requests to
    /// https://api.arbiscan.io, so the API key must be for https://api.arbiscan.io
    network: Network,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Network {
    Arbitrum,
    Aurora,
    Avalanche,
    Bsc,
    Bttc,
    Celo,
    Clv,
    Cronos,
    Ethereum,
    Fantom,
    Heco,
    Optimism,
    Moonbeam,
    Moonriver,
    Polygon,
}

fn main() -> Result<()> {
    info!("start up");

    let args = Args::parse();

    let network = match args.network {
        Network::Arbitrum => "arbitrum",
        Network::Aurora => "aurora",
        Network::Avalanche => "avalanche",
        Network::Bsc => "bsc",
        Network::Bttc => "bttc",
        Network::Celo => "celo",
        Network::Clv => "clv",
        Network::Cronos => "cronos",
        Network::Ethereum => "ethereum",
        Network::Fantom => "fantom",
        Network::Heco => "heco",
        Network::Optimism => "optimism",
        Network::Moonbeam => "moonbeam",
        Network::Moonriver => "moonriver",
        Network::Polygon => "polygon",
    };

    let http_client = ReqwestClient::new();

    let contracts_client = Client::new(args.api_key, args.api_url, http_client, network)
        .context("failed to create HTTP client")?;

    let contracts_service = Service::new(&contracts_client);

    let contracts = contracts_service
        .get_contracts(&args.contract_address)
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

        let contract_dir = Path::new(&args.files_dest_path)
            .join(network)
            .join(&args.contract_address)
            .join(p.parent().unwrap());
        fs::create_dir_all(&contract_dir).context("failed to create contracts dir")?;

        // create Solidity file
        let contract_file_path = contract_dir.join(file_name);
        fs::write(contract_file_path, &c.code)
            .context("failed to crate and write to solidity file")?;
    }

    Ok(())
}
