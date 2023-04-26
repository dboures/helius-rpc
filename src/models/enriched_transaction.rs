use serde::{Deserialize};
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};
use solana_program::{clock::UnixTimestamp, pubkey::Pubkey, slot_history::Slot};
use solana_sdk::{
    commitment_config::CommitmentLevel, signature::Signature,
};

use super::{
    enums::{TransactionSource, TransactionType},
    nft::{CompressedNftEvent, NftEvent},
};

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

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedTransaction {
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub source: TransactionSource,
    pub fee: u64,
    pub fee_payer: String,
    pub signature: String,
    pub slot: Slot,
    pub timestamp: Option<UnixTimestamp>,
    pub native_transfers: Vec<NativeTransfer>,
    pub token_transfers: Vec<TokenTransfer>,
    pub account_data: Vec<EnrichedAccountData>,
    pub transaction_error: Option<EnrichedError>,
    pub instructions: Vec<EnrichedInstruction>,
    pub events: EnrichedEvents,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedEvents {
    pub nft: Option<NftEvent>,
    pub swap: Option<SwapEvent>,
    pub compressed: Option<CompressedNftEvent>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeTransfer {
    pub from_user_account: String,
    pub to_user_account: String,
    pub amount: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenTransfer {
    pub from_user_account: String,
    pub to_user_account: String,
    pub from_token_account: String,
    pub to_token_account: String,
    pub token_amount: u64,
    pub mint: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedAccountData {
    pub account: String,
    pub native_balance_change: i128,
    pub token_balance_changes: Vec<EnrichedTokenBalanceChange>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedInstruction {
    pub accounts: Vec<String>,
    pub data: String,
    pub program_id: String,
    pub inner_instructions: Vec<EnrichedInnerInstruction>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedInnerInstruction {
    pub accounts: Vec<String>,
    pub data: String,
    pub program_id: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedTokenBalanceChange {
    pub user_account: String,
    pub token_account: String,
    pub mint: String,
    pub raw_token_amount: RawTokenAmount,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawTokenAmount {
    pub token_amount: String,
    pub decimals: u8,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedError {
    pub error: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapEvent {
    pub error: String,
}
