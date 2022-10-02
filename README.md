# Etherscan CLI


`etherscan` is a CLI app to interact with etherscan.io API or other blockchain explorers built with etherscan.io.
Like the ones listed [here](https://etherscan.io/eaas).

Currently it supports downloading contracts files only and only with verified contracts for which the source code has
been upload to the blockchain explorer.

This is project I used to learn Rust, if you have any improvements recommendations or find any bugs please create an
issue.


## Install

### Download

```bash
curl -LO https://github.com/dragossutu/etherscan/releases/download/1.9.0/etherscan-linux-amd64-1.9.0 -o etherscan \
  && chmod +x etherscan \
  && sudo mv etherscan /usr/local/bin
```

### Build from source

```bash
git clone https://github.com/dragossutu/etherscan.git
cd etherscan
cargo build --release
```


## How to use

1. Create an API key for the block explorer you want to use the CLI for. (requires an account)

2. (Optional) Export the API key as an environment variable ETHERSCAN_API_KEY.
Can be passed as CLI flag outherwise
```bash
export ETHERSCAN_API_KEY=your_api_key_here
```

3. Download contract files
```bash
etherscan 0x34d85c9CDeB23FA97cb08333b511ac86E1C4E258
```

Unless you used the `--files-dest-path` flag, the downloaded contracts files will be in a local folder
```
./contracts/[BLOCKCHAIN NAME]/[CONTRACT ADDRESS]
```

For the example above the folder would be `./contracts/ethereum/0x34d85c9CDeB23FA97cb08333b511ac86E1C4E258`

### Usage

```bash
$ etherscan --help
A CLI app to interact with etherscan.io API or other blockchain explorers built with etherscan.io

USAGE:
    etherscan [OPTIONS] --api-key <API_KEY> <CONTRACT_ADDRESS>

ARGS:
    <CONTRACT_ADDRESS>
            Address of the contract to download files for

OPTIONS:
    -d, --files-dest-path <FILES_DEST_PATH>
            Local path to the folder where the contract's files will be created

            Folder will be created if it doesn't exist

            [default: ./contracts]

    -h, --help
            Print help information

    -k, --api-key <API_KEY>
            The API key for the block explorer

            It will be read from the environment variable ETHERSCAN_API_KEY first

            [env: ETHERSCAN_API_KEY=]

    -n, --network <NETWORK>
            The name of the network

            Must match the block explorer that the API key is for. e.g if network = arbitrum, the
            CLI will make requests to https://api.arbiscan.io, so the API key must be for
            https://api.arbiscan.io

            [default: ethereum]
            [possible values: arbitrum, aurora, avalanche, bsc, bttc, celo, clv, cronos, ethereum,
            fantom, heco, optimism, moonbeam, moonriver, polygon]

    -u, --api-url <API_URL>
            Used for tests only. The URL of the block explorer's API

            e.g. https://api.etherscan.io. If passed in, the `network` argument is ignored

    -V, --version
            Print version information
```

## Supported blockchains explorers

- Arbitrum: [Mainnet](https://arbiscan.io)
- Aurora: [Mainnet](https://aurorascan.dev)
- Avalanche: [Mainnet](https://snowtrace.io)
- Bsc: [Mainnet](https://bscscan.com)
- Bttc: [Mainnet](https://bttcscan.com)
- Celo: [Mainnet](https://celoscan.xyz)
- Clv: [Mainnet](https://clvscan.com)
- Cronos: [Mainnet](https://cronoscan.com)
- Ethereum: [Mainnet](https://etherscan.io)
- Fantom: [Mainnet](https://ftmscan.com)
- Heco: [Mainnet](https://hecoinfo.com)
- Optimism: [Mainnet](https://optimistic.etherscan.io)
- Moonbeam: [Mainnet](https://moonbeam.moonscan.io)
- Moonriver: [Mainnet](https://moonriver.moonscan.io)
- olygon: [Mainnet](https://polygonscan.com)


## Contributing

Thanks for your interest in contributing to the etherscan CLI!

If you find a bug or want to add a feature please:

- check existing issues to verify there isn't another PR for the same bug/feature
- create an issue
- create a new branch
- make sure you have [pre-commit](https://pre-commit.com/#installation) installed and configured for this repo
```bash
cd etherscan
pre-commit install
```
- make your change, add tests and make sure tests pass. The codebase uses
[semantic-release](https://github.com/semantic-release/semantic-release), so please follow the [angular commit message format](https://github.com/semantic-release/semantic-release#commit-message-format) so that semantic-release can create a new release and update changelog from your commit messages
- create a pull request
