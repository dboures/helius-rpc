use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};
use solana_program::{pubkey::Pubkey, slot_history::Slot, clock::UnixTimestamp};
use solana_sdk::{
    commitment_config::CommitmentLevel, signature::Signature, transaction::TransactionError,
};
use solana_transaction_status::{
    option_serializer::OptionSerializer, Rewards, UiInnerInstructions, UiLoadedAddresses,
    UiTransactionReturnData, UiTransactionTokenBalance,
};

use super::enriched_transaction::{RequestConfig};

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawTransaction {
    pub slot: Slot,
    pub block_time: Option<UnixTimestamp>,
    pub transaction: InnerTransaction,
    pub meta: UiTransactionStatusMeta,
    pub version: Option<String>,
    pub index_within_block: Option<u64>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerTransaction {
    pub signatures: Vec<String>,
    pub message: InnerTransactionMessage,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerTransactionMessage {
    pub account_keys: Vec<String>,
    pub header: Value,
    pub recent_blockhash: String,
    pub instructions: Vec<InnerInstruction>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerInstruction {
    pub program_id_index: usize,
    pub accounts: Vec<usize>,
    pub data: String
}

#[derive(Debug, Default)]
pub struct GetRawTransactionsRequestConfig {
    pub address: Pubkey,
    pub before: Option<Signature>,
    pub until: Option<Signature>,
    pub limit: Option<usize>,
    pub commitment: Option<CommitmentLevel>,
}
impl GetRawTransactionsRequestConfig {
    pub fn generate_query_parameters(
        &self,
        api_key: String,
    ) -> ClientResult<Vec<(String, String)>> {
        let config = RequestConfig {
            address: self.address,
            before: self.before,
            until: self.until,
            limit: self.limit,
            source: None,
            transaction_type: None,
            commitment: self.commitment,
        };
        config.generate_query_parameters(api_key)
    }
}

/// A duplicate representation of TransactionStatusMeta with `err` field. Copied from solana-transactions-status crate, but without the status field.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTransactionStatusMeta {
    pub err: Option<TransactionError>,
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub inner_instructions: OptionSerializer<Vec<UiInnerInstructions>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub log_messages: OptionSerializer<Vec<String>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub pre_token_balances: OptionSerializer<Vec<UiTransactionTokenBalance>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub post_token_balances: OptionSerializer<Vec<UiTransactionTokenBalance>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub rewards: OptionSerializer<Rewards>,
    #[serde(
        default = "OptionSerializer::skip",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub loaded_addresses: OptionSerializer<UiLoadedAddresses>,
    #[serde(
        default = "OptionSerializer::skip",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub return_data: OptionSerializer<UiTransactionReturnData>,
    #[serde(
        default = "OptionSerializer::skip",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub compute_units_consumed: OptionSerializer<u64>,
}
