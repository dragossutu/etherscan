use crate::Request;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

pub(crate) trait Contracts {
    fn get_contracts(&self, contract_address: &str) -> Result<Vec<Contract>, Box<dyn Error>>;
}

pub(crate) struct Service<'a, C>
where
    C: Request,
{
    client: &'a C,
}

impl<'a, C> Service<'a, C>
where
    C: Request,
{
    pub(crate) fn new(client: &C) -> Service<C> {
        Service { client }
    }
}

impl<'a, C> Contracts for Service<'a, C>
where
    C: Request,
{
    fn get_contracts(&self, contract_address: &str) -> Result<Vec<Contract>, Box<dyn Error>> {
        let contracts_info = self.client.get_source_code(contract_address)?;

        let mut contracts = Vec::new();

        for c in contracts_info.iter() {
            let mut source_code_raw = c.source_code.strip_prefix("{").unwrap_or(&c.source_code);

            source_code_raw = source_code_raw.strip_suffix("}").unwrap_or(source_code_raw);

            let s: SourceCode = serde_json::from_str(source_code_raw)?;

            for (contract_path, contract_code) in s.sources {
                contracts.push(Contract {
                    code: contract_code.content,
                    path: contract_path,
                });
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

pub(crate) struct Contract {
    pub(crate) code: String,
    pub(crate) path: String,
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

        mock_client.expect_get_source_code()
            .with(eq(contract_address))
            .times(1)
            .returning(|_| Ok(vec![ContractInfo{ source_code: source_code.to_string() }]));

        let service = Service::new(&mock_client);

        let r = service.get_contracts(contract_address);
        assert!(r.is_ok(), "Result returned by get_contracts() is not Ok");
        let contracts = r.unwrap();
        assert_eq!(contracts.len(), 1, "returned Vec<Contract> didn't have the expected length");
        let contract = contracts.get(0).unwrap();
        assert_eq!(contract.path, "main.sol", "contract path didn't match");
        assert_eq!(contract.code, "pragma solidity 0.8.14;", "contract code didn't match");
    }
}
