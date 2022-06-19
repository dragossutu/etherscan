use anyhow::{anyhow, Result};
use reqwest::blocking::Client as ReqwestClient;
use serde::Deserialize;
use std::vec::Vec;

#[cfg_attr(test, mockall::automock)]
pub(crate) trait Request {
    fn get_source_code(&self, contract_address: &str) -> Result<Vec<ContractInfo>>;
}

pub(crate) struct Client {
    api_key: String,
    api_url: String,
    http_client: ReqwestClient,
}

impl Client {
    pub(crate) fn new(api_key: String, api_url: String, http_client: ReqwestClient) -> Client {
        Client {
            api_key,
            api_url,
            http_client,
        }
    }
}

impl Request for Client {
    fn get_source_code(&self, contract_address: &str) -> Result<Vec<ContractInfo>> {
        let url = format!(
            "{}/api?module=contract&action=getsourcecode&address={}&apikey={}",
            self.api_url, contract_address, self.api_key,
        );

        let res = self.http_client.get(&url).send()?;

        if !res.status().is_success() {
            return Err(anyhow!(
                "non-success response http code: {}",
                res.status().as_str()
            ));
        }

        let body: ResponseBody = res.json()?;

        Ok(body.result)
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
    #[serde(alias = "ContractName")]
    pub(crate) contract_name: String,
    #[serde(alias = "SourceCode")]
    pub(crate) source_code: String,
}
