use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    commitment_config::CommitmentLevel, signature::Signature, transaction::TransactionError,
};
use solana_transaction_status::{
    option_serializer::OptionSerializer, Rewards, UiInnerInstructions, UiLoadedAddresses,
    UiTransactionReturnData, UiTransactionTokenBalance,
};

use super::enums::{TransactionSource, TransactionType};


#[derive(Debug, Default)]
pub struct RequestConfig {
    pub address: Pubkey,
    pub before: Option<Signature>,
    pub until: Option<Signature>,
    pub limit: Option<usize>,
    pub source: Option<TransactionSource>,
    pub transaction_type: Option<TransactionType>,
    pub commitment: Option<CommitmentLevel>,
}
impl RequestConfig {
    pub fn generate_query_parameters(
        &self,
        api_key: String,
    ) -> ClientResult<Vec<(String, String)>> {
        let mut query_params = vec![
            ("address".to_string(), self.address.to_string()),
            ("api-key".to_string(), api_key),
        ];
        if self.before.is_some() {
            query_params.push(("before".to_string(), self.before.unwrap().to_string()));
        }
        if self.until.is_some() {
            query_params.push(("until".to_string(), self.until.unwrap().to_string()));
        }
        if self.limit.is_some() {
            query_params.push(("limit".to_string(), self.limit.unwrap().to_string()));
        }
        if self.source.is_some() {
            query_params.push(("source".to_string(), self.source.unwrap().to_string()));
        }
        if self.transaction_type.is_some() {
            query_params.push((
                "type".to_string(),
                self.transaction_type.unwrap().to_string(),
            ));
        }

        match self.commitment {
            Some(CommitmentLevel::Confirmed) => {
                query_params.push(("commitment".to_string(), "confirmed".to_string()));
            }
            Some(CommitmentLevel::Finalized) => {
                query_params.push(("commitment".to_string(), "finalized".to_string()));
            }
            None => {}
            _ => {
                return Err(ClientError::from(ClientErrorKind::Custom(
                    "Only Confirmed and Finalized commitments are supported by this API"
                        .to_string(),
                )));
            }
        }
        Ok(query_params)
    }
}

// #[derive(Clone, Debug, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct NativeTransfer {
//     from_user_account: String,
//     to_user_account: String,
//     amount: u64,
// }

// #[derive(Clone, Debug, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TokenTransfer {
//     pub from_user_account: String,
//     pub to_user_account: String,
//     pub from_token_account: String,
//     pub to_token_account: String,
//     pub token_amount: u64,
//     pub mint: String,
// }

// #[derive(Clone, Debug, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct InnerInstruction {
//     pub accounts: Vec<String>,
//     pub data: String,
//     pub program_id: String,
// }

// #[derive(Clone, Debug, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Instruction {
//     pub accounts: Vec<String>,
//     pub data: String,
//     pub program_id: String,
// }

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
