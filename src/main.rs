use anyhow::Result;
use clap::Parser;
use log::info;

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

    scancli::go(
        args.api_key,
        args.api_url,
        args.contract_address,
        args.files_dest_path,
        network,
    )
}
