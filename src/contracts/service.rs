use crate::Request;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

pub trait Contracts {
    fn get_contracts(&self, contract_address: &str) -> Result<Vec<Contract>>;
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
    fn get_contracts(&self, contract_address: &str) -> Result<Vec<Contract>> {
        let c = self
            .client
            .get_source_code(contract_address)
            .context("client failed to get source code")?;

        let mut contracts = Vec::new();

        let mut source_code_raw = c.source_code.clone();

        if source_code_raw.starts_with("{{") {
            source_code_raw = source_code_raw.strip_prefix('{').unwrap().to_string();
        }

        if source_code_raw.ends_with("}}") {
            source_code_raw = source_code_raw.strip_suffix('}').unwrap().to_string();
        }

        // `source_code_raw` is a string containing either:
        // - all the contract code, that uploaded from a single file
        // - or a JSON with the contract code split into multiple files
        match serde_json::from_str(&source_code_raw) {
            Err(e) => {
                if e.is_syntax() && e.line() == 1 && e.column() == 1 {
                    // TODO: log here
                    contracts.push(Contract {
                        code: source_code_raw,
                        path: c.contract_name,
                    });
                } else {
                    return Err(anyhow::Error::new(e)
                        .context("failed to deserialize JSON contract source code"));
                }
            }
            Ok(r) => {
                let s: SourceCode = r;
                for (contract_path, contract_code) in s.sources {
                    contracts.push(Contract {
                        code: contract_code.content,
                        path: contract_path,
                    });
                }
            }
        }

        Ok(contracts)
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

pub struct Contract {
    pub code: String,
    pub path: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::client::{ContractInfo, MockRequest};
    use mockall::predicate::*;

    #[test]
    fn gets_contracts_successfully() {
        let mut mock_client = MockRequest::new();
        let contract_address = "0x1234567890abcdef";
        let source_code = r#"{{
            "sources": {
                "main.sol": {
                    "content": "pragma solidity 0.8.14;"
                }
            }
        }}"#;

        mock_client
            .expect_get_source_code()
            .with(eq(contract_address))
            .times(1)
            .returning(|_| {
                Ok(ContractInfo {
                    contract_name: "MyContract".to_string(),
                    source_code: source_code.to_string(),
                })
            });

        let service = Service::new(&mock_client);

        let r = service.get_contracts(contract_address);
        assert!(r.is_ok(), "Result returned by get_contracts() is not Ok");
        let contracts = r.unwrap();
        assert_eq!(
            contracts.len(),
            1,
            "returned Vec<Contract> didn't have the expected length"
        );
        let contract = contracts.get(0).unwrap();
        assert_eq!(contract.path, "main.sol", "contract path didn't match");
        assert_eq!(
            contract.code, "pragma solidity 0.8.14;",
            "contract code didn't match"
        );
    }
}
