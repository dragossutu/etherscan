use crate::Request;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

pub trait Contracts {
    fn get_contracts(&self, contract_address: &str) -> Result<Contract>;
}

pub struct Service<'a, C>
where
    C: Request,
{
    client: &'a C,
}

impl<'a, C> Service<'a, C>
where
    C: Request,
{
    pub fn new(client: &C) -> Service<C> {
        Service { client }
    }
}

impl<'a, C> Contracts for Service<'a, C>
where
    C: Request,
{
    fn get_contracts(&self, contract_address: &str) -> Result<Contract> {
        let c = self
            .client
            .get_contract_info(contract_address)
            .context("client failed to get contract info")?;

        let mut source_code_raw = c.source_code.clone();

        if source_code_raw.starts_with("{{") {
            source_code_raw = source_code_raw.strip_prefix('{').unwrap().to_string();
        }

        if source_code_raw.ends_with("}}") {
            source_code_raw = source_code_raw.strip_suffix('}').unwrap().to_string();
        }

        let mut parts = Vec::new();

        // `source_code_raw` is a string containing either:
        // - all the contract code, that was uploaded from a single file
        // - or a JSON with the contract code split into multiple files
        match serde_json::from_str(&source_code_raw) {
            Err(e) => {
                if e.is_syntax() && e.line() == 1 && e.column() == 1 {
                    // TODO: log here
                    parts.push(ContractPart {
                        code: source_code_raw,
                        path: c.contract_name.clone(),
                    });
                } else {
                    return Err(anyhow::Error::new(e)
                        .context("failed to deserialize JSON contract source code"));
                }
            }
            Ok(r) => {
                let s: SourceCode = r;
                for (contract_path, contract_code) in s.sources {
                    parts.push(ContractPart {
                        code: contract_code.content,
                        path: contract_path,
                    });
                }
            }
        }

        let c = Contract {
            name: c.contract_name,
            parts,
        };

        Ok(c)
    }
}

#[derive(Deserialize)]
struct SourceCode {
    sources: HashMap<String, Source>,
}

#[derive(Deserialize)]
struct Source {
    content: String,
}

#[derive(Debug)]
pub struct ContractPart {
    pub code: String,
    pub path: String,
}

#[derive(Debug)]
pub struct Contract {
    pub(crate) name: String,
    pub(crate) parts: Vec<ContractPart>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::client::{ContractInfo, MockRequest};
    use anyhow::anyhow;
    use mockall::predicate::*;
    use pretty_assertions::assert_str_eq;

    const CONTRACT_ADDRESS: &str = "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984";

    #[test]
    fn gets_contracts_successfully() {
        // given
        let mut mock_client = MockRequest::new();
        let source_code = r#"{{
            "sources": {
                "main.sol": {
                    "content": "pragma solidity 0.8.14;"
                }
            }
        }}"#;

        mock_client
            .expect_get_contract_info()
            .with(eq(CONTRACT_ADDRESS))
            .times(1)
            .returning(|_| {
                Ok(ContractInfo {
                    contract_name: "MyContract".to_string(),
                    source_code: source_code.to_string(),
                })
            });

        let service = Service::new(&mock_client);

        // when
        let r = service.get_contracts(CONTRACT_ADDRESS);

        // then
        assert!(r.is_ok(), "result is not ok");
        let contract = r.unwrap();
        assert_eq!(
            contract.parts.len(),
            1,
            "contract.parts length is not equal to expected"
        );
        let part = contract.parts.get(0).unwrap();
        assert_eq!(
            part.path, "main.sol",
            "contract path is not equal to expected"
        );
        assert_eq!(
            part.code, "pragma solidity 0.8.14;",
            "contract code is not equal to expected"
        );
    }

    #[test]
    fn returns_error_when_client_returns_error() {
        // given
        let mut mock_client = MockRequest::new();

        mock_client
            .expect_get_contract_info()
            .with(eq(CONTRACT_ADDRESS))
            .times(1)
            .returning(|_| Err(anyhow!("received non-success response http code: 500")));

        let service = Service::new(&mock_client);

        // when
        let r = service.get_contracts(CONTRACT_ADDRESS);

        // then
        assert!(r.is_err(), "result is not error");
        assert_str_eq!(
            r.unwrap_err().to_string(),
            "client failed to get contract info"
        )
    }
}
