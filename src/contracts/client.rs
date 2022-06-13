use serde::Deserialize;
use std::error::Error;
use std::vec::Vec;

#[cfg_attr(test, mockall::automock)]
pub(crate) trait Request {
    fn get_source_code(&self, contract_address: &str) -> Result<Vec<ContractInfo>, Box<dyn Error>>;
}

pub(crate) struct Client {
    api_key: String,
}

impl Client {
    pub(crate) fn new(api_key: String) -> Client {
        Client { api_key }
    }
}

impl Request for Client {
    fn get_source_code(&self, contract_address: &str) -> Result<Vec<ContractInfo>, Box<dyn Error>> {
        let url = format!(
            "https://api.etherscan.io/api?module=contract&action=getsourcecode&address={}&apikey={}",
            contract_address,
            self.api_key,
        );

        let res = reqwest::blocking::get(&url)?;

        if !res.status().is_success() {
            return Err(From::from(format!(
                "request response http code: {}",
                res.status().as_str()
            )));
        }

        let body = res.text()?;
        let body_json: ResponseBody = serde_json::from_str(&body)?;

        Ok(body_json.result)
    }
}

#[derive(Deserialize)]
struct ResponseBody {
    result: Vec<ContractInfo>,
}

#[derive(Deserialize)]
pub(crate) struct ContractInfo {
    // #[serde(alias = "ABI")]
    // abi: String,
    // #[serde(alias = "ContractName")]
    // contract_name: String,
    #[serde(alias = "SourceCode")]
    pub(crate) source_code: String,
}
