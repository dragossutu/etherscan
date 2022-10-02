use crate::contracts::service::Contract;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

pub trait Files {
    fn create_contract_files(
        &self,
        files_dest_path: &str,
        network_name: &str,
        contract_address: &str,
        contract: Contract,
    ) -> Result<()>;
}

pub struct Service {}

impl Service {
    pub fn new() -> Service {
        Service {}
    }
}

impl Files for Service {
    fn create_contract_files(
        &self,
        files_dest_path: &str,
        network_name: &str,
        contract_address: &str,
        contract: Contract,
    ) -> Result<()> {
        for c in contract.parts.iter() {
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

            let mut contract_dir_name_builder = String::from(&contract.name);
            contract_dir_name_builder.push('-');
            contract_dir_name_builder.push_str(contract_address);

            let contract_path = Path::new(files_dest_path)
                .join(network_name)
                .join(contract_dir_name_builder.as_str())
                .join(p.parent().unwrap());

            fs::create_dir_all(&contract_path).context("failed to create contracts dir")?;

            // create Solidity file
            let contract_file_path = contract_path.join(file_name);
            fs::write(contract_file_path, &c.code)
                .context("failed to crate and write to solidity file")?;
        }

        Ok(())
    }
}
