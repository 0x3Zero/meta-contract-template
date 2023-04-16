#![allow(improper_ctypes)]

mod data;
mod defaults;
mod types;

use data::DataStructFork;
use defaults::DEFAULT_IPFS_MULTIADDR;
use defaults::DEFAULT_TIMEOUT_SEC;
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use types::MetaContract;
use types::Metadata;
use types::Transaction;
use types::{FinalMetadata, MetaContractResult};

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn on_execute(
    contract: MetaContract,
    metadatas: Vec<Metadata>,
    transaction: Transaction,
) -> MetaContractResult {
    let mut finals: Vec<FinalMetadata> = vec![];

    // Only 10 beat
    if metadatas.len() > 13 {
        return MetaContractResult {
            result: false,
            metadatas: finals,
            error_string: "Can not be more than 10 beats".to_string(),
        };
    }

    if metadatas.len() <= 0 {
        finals.push(FinalMetadata {
            public_key: contract.public_key.clone(),
            alias: "name".to_string(),
            content: format!("Collabeat #{}", transaction.token_id),
        });

        finals.push(FinalMetadata {
            public_key: contract.public_key.clone(),
            alias: "description".to_string(),
            content: "Co-Create, Collaborate and Own The Beat".to_string(),
        });

        finals.push(FinalMetadata {
            public_key: contract.public_key.clone(),
            alias: "image".to_string(),
            content: "ipfs://".to_string(),
        });
    }

    finals.push(FinalMetadata {
        public_key: transaction.public_key,
        alias: transaction.alias,
        content: transaction.data,
    });

    MetaContractResult {
        result: true,
        metadatas: finals,
        error_string: "".to_string(),
    }
}

#[marine]
pub fn on_clone() -> bool {
    return true;
}

#[marine]
pub fn on_mint(contract: MetaContract, token_id: String, ipfs_multiaddr: String, cid: String) -> MetaContractResult {
    let mut finals: Vec<FinalMetadata> = vec![];

    finals.push(FinalMetadata {
        public_key: contract.public_key.clone(),
        alias: "name".to_string(),
        content: format!("Collabeat #{}", token_id),
    });

    finals.push(FinalMetadata {
        public_key: contract.public_key.clone(),
        alias: "description".to_string(),
        content: "Co-Create, Collaborate and Own The Beat".to_string(),
    });

    finals.push(FinalMetadata {
        public_key: contract.public_key.clone(),
        alias: "image".to_string(),
        content: "ipfs://".to_string(),
    });

    // extract out data
    if cid.len() > 0 {
        let datasets = get(cid, ipfs_multiaddr, 0);
        let result: Result<Vec<DataStructFork>, serde_json::Error> =
            serde_json::from_str(&datasets);

        match result {
            Ok(datas) => {
                for data in datas {
                    finals.push(FinalMetadata {
                        public_key: data.owner,
                        alias: "".to_string(),
                        content: data.cid,
                    });
                }
            }
            Err(_) => {
                return MetaContractResult {
                    result: false,
                    metadatas: Vec::new(),
                    error_string: "Invalid data structure".to_string(),
                };
            }
        }
    }

    MetaContractResult {
        result: true,
        metadatas: finals,
        error_string: "".to_string(),
    }
}

/**
 * Get data from ipfs
 */
fn get(hash: String, api_multiaddr: String, timeout_sec: u64) -> String {
    let address: String;
    let t;

    if api_multiaddr.is_empty() {
        address = DEFAULT_IPFS_MULTIADDR.to_string();
    } else {
        address = api_multiaddr;
    }

    if timeout_sec == 0 {
        t = DEFAULT_TIMEOUT_SEC;
    } else {
        t = timeout_sec;
    }

    let args = vec![String::from("dag"), String::from("get"), hash];

    let cmd = make_cmd_args(args, address, t);

    let result = ipfs(cmd);

    String::from_utf8(result.stdout).unwrap()
}

pub fn make_cmd_args(args: Vec<String>, api_multiaddr: String, timeout_sec: u64) -> Vec<String> {
    args.into_iter()
        .chain(vec![
            String::from("--timeout"),
            get_timeout_string(timeout_sec),
            String::from("--api"),
            api_multiaddr,
        ])
        .collect()
}

#[inline]
pub fn get_timeout_string(timeout: u64) -> String {
    format!("{}s", timeout)
}

// Service
// - curl

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;
}
