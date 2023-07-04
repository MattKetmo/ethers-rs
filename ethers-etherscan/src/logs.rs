use crate::{Client, Response, Result};
use ethers_core::{
    abi::Address,
    types::{serde_helpers::*, BlockNumber, Bytes, H256, U256},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
};

/// The raw response from the event logs API endpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    #[serde(deserialize_with = "deserialize_stringified_block_number")]
    pub block_number: BlockNumber,
    pub block_hash: H256,
    pub time_stamp: U256,
    pub transaction_hash: H256,
    #[serde(deserialize_with = "deserialize_stringified_u64_opt")]
    pub transaction_index: Option<u64>,
    #[serde(deserialize_with = "deserialize_stringified_u64_opt")]
    pub log_index: Option<u64>,
    pub address: Address,
    #[serde(deserialize_with = "deserialize_stringified_numeric")]
    pub gas_price: U256,
    #[serde(deserialize_with = "deserialize_stringified_numeric")]
    pub gas_used: U256,
    pub topics: Vec<H256>,
    pub data: Bytes,
}

// fn deserialize_hex_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let s: &str = serde::Deserialize::deserialize(deserializer)?;
//     u64::from_str_radix(s.trim_start_matches("0x"), 16)
//         .map(|num| num.to_string())
//         .map_err(serde::de::Error::custom)
// }

/// Common optional arguments for the log list API endpoints
#[derive(Clone, Copy, Debug)]
pub struct LogListParams {
    pub from_block: u64,
    pub to_block: u64,
    pub page: u64,
    pub offset: u64,
}

impl LogListParams {
    pub fn new(start_block: u64, end_block: u64, page: u64, offset: u64) -> Self {
        Self { from_block: start_block, to_block: end_block, page, offset }
    }
}

impl Default for LogListParams {
    fn default() -> Self {
        Self { from_block: 0, to_block: 99999999, page: 0, offset: 10000 }
    }
}

impl From<LogListParams> for HashMap<&'static str, String> {
    fn from(tx_params: LogListParams) -> Self {
        let mut params = HashMap::new();
        params.insert("fromBlock", tx_params.from_block.to_string());
        params.insert("toBlock", tx_params.to_block.to_string());
        params.insert("page", tx_params.page.to_string());
        params.insert("offset", tx_params.offset.to_string());
        params
    }
}

impl Client {
    pub async fn get_logs(
        &self,
        address: Option<Address>,
        params: Option<LogListParams>,
    ) -> Result<Vec<Log>> {
        let mut logs_params: HashMap<&str, String> = params.unwrap_or_default().into();

        match address {
            Some(address) => logs_params.insert("address", format!("{address:?}")),
            None => None,
        };

        let query = self.create_query("logs", "getLogs", logs_params);
        let response: Response<Vec<Log>> = self.get_json(&query).await?;

        Ok(response.result)
    }
}
