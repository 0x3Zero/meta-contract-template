#![allow(improper_ctypes)]

mod types;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
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
pub fn on_mint(contract: MetaContract, transaction: Transaction) -> MetaContractResult {
    let mut finals: Vec<FinalMetadata> = vec![];

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

// Service
// - curl
