use anyhow::{Context, Result};
use clap::{ArgEnum, Parser};
use log::info;
use reqwest::blocking::Client as ReqwestClient;

mod contracts;

use contracts::client::{Client, Request};
use contracts::service::{Contracts, Service as ContractsService};

mod files;
use files::{Files, Service as FilesService};

pub const CONTRACTS_DEST_DIR: &str = "./contracts";

#[derive(Parser)]
#[clap(about, version)]
struct Args {
    #[clap(env = "ETHERSCAN_API_KEY", long, short = 'k')]
    /// The API key for the block explorer
    ///
    /// It will be read from the environment variable ETHERSCAN_API_KEY first
    api_key: String,

    #[clap(long, short = 'u')]
    /// Used for tests only. The URL of the block explorer's API
    ///
    /// e.g. https://api.etherscan.io. If passed in, the `network` argument is ignored
    api_url: Option<String>,

    /// Address of the contract to download files for
    contract_address: String,

    #[clap(default_value = "./contracts", long, short = 'd')]
    /// Local path to the folder where the contract's files will be created
    ///
    /// Folder will be created if it doesn't exist
    files_dest_path: String,

    #[clap(arg_enum, default_value_t = Network::Ethereum, long, short, value_parser)]
    /// The name of the network
    ///
    /// Must match the block explorer that the API key is for.
    /// e.g if network = arbitrum, the CLI will make requests to https://api.arbiscan.io,
    /// so the API key must be for https://api.arbiscan.io
    network: Network,
}

#[derive(ArgEnum, Clone)]
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

    let (network_name, mut api_url) = match args.network {
        Network::Arbitrum => ("arbitrum", "https://api.arbiscan.io"),
        Network::Aurora => ("aurora", "https://api.aurorascan.dev"),
        Network::Avalanche => ("avalanche", "https://api.snowtrace.io"),
        Network::Bsc => ("bsc", "https://api.bscscan.com"),
        Network::Bttc => ("bttc", "https://api.bttcscan.com"),
        Network::Celo => ("celo", "https://api.celoscan.xyz"),
        Network::Clv => ("clv", "https://api.clvscan.com"),
        Network::Cronos => ("cronos", "https://api.cronoscan.com"),
        Network::Ethereum => ("ethereum", "https://api.etherscan.io"),
        Network::Fantom => ("fantom", "https://api.ftmscan.com"),
        Network::Heco => ("heco", "https://api.hecoinfo.com"),
        Network::Optimism => ("optimism", "https://api-optimistic.etherscan.io"),
        Network::Moonbeam => ("moonbeam", "https://api-moonbeam.moonscan.io"),
        Network::Moonriver => ("moonriver", "https://api-moonriver.moonscan.io"),
        Network::Polygon => ("polygon", "https://api.polygonscan.com"),
    };

    if args.api_url.is_some() {
        api_url = args.api_url.as_deref().unwrap();
    }

    let http_client = ReqwestClient::new();

    let contracts_client = Client::new(args.api_key.as_str(), api_url, http_client);

    let contracts_service = ContractsService::new(&contracts_client);

    let files_service = FilesService::new();

    let contracts = contracts_service
        .get_contracts(&args.contract_address)
        .context("contracts_service failed to get contracts")?;

    files_service.create_contract_files(
        &args.files_dest_path,
        network_name,
        &args.contract_address,
        contracts,
    )
}
