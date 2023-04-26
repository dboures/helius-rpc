use serde::Deserialize;
use serde_json::{json, Value};

use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};

use super::{
    enriched_transaction::{NativeTransfer, TokenTransfer},
    enums::{CompressedNftEventType, NftEventType, SaleType, TokenStandard, TransactionSource},
};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveListing {
    pub transaction_signature: String,
    pub marketplace: String,
    pub amount: u64,
    pub seller: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NftMetadata {
    pub mint: String,
    pub name: String,
    pub burned: bool,
    pub first_verified_creator: String,
    pub verified_collection_address: String,
    pub active_listings: Vec<ActiveListing>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompressedNftEvent {
    event_type: CompressedNftEventType,
    tree_id: String,
    asset_id: String,
    leaf_index: u64,
    instruction_index: u64,
    inner_instruction_index: u64,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NftEvent {
    pub amount: i128, // Sometimes has negatives, although it should be lamports?
    pub fee: u64,
    pub fee_payer: String,
    pub signature: String,
    pub slot: u64,
    pub timestamp: u64,
    #[serde(rename = "type")]
    pub event_type: NftEventType,
    pub buyer: String,
    pub seller: String,
    pub staker: String,
    pub nfts: Vec<NftToken>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NftEventV2 {
    #[serde(rename = "type")]
    pub event_type: NftEventType,
    pub source: TransactionSource,
    pub amount: i128, // Sometimes has negatives, although it should be lamports?
    pub fee: u64,
    pub fee_payer: String,
    pub signature: String,
    pub slot: u64,
    pub timestamp: u64,
    pub sale_type: SaleType,
    pub buyer: String,
    pub seller: String,
    pub staker: String,
    pub nfts: Value,
    pub native_transfers: Vec<NativeTransfer>,
    pub token_transfers: Vec<TokenTransfer>,
    pub pagination_token: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NftToken {
    pub mint: String,
    pub token_standard: TokenStandard,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MintListResult {
    pub mint: String,
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalancesResponse {
    pub native_balance: u64,
    pub tokens: Vec<TokenBalance>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MintListResponse {
    pub result: Vec<MintListResult>,
    pub pagination_token: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NftResponse {
    pub number_of_pages: usize,
    pub nfts: NftInfo,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename = "nft")]
#[serde(rename_all = "camelCase")]
pub struct NftInfo {
    pub name: String,
    pub token_address: String,
    pub collection_address: String,
    pub collection_name: String,
    pub image_url: String,
    pub traits: Vec<Trait>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Trait {
    pub trait_type: String,
    pub value: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub token_account: String,
    pub mint: String,
    pub amount: u64,
    pub decimals: u8,
}

/// Request parameters for the `https://api.helius.xyz/v1/mintlist` endpoint. The API only accepts one of verified_collection_addresses or first_verified_creators, not both.
#[derive(Debug, Default)]
pub struct MintListRequestConfig {
    pub verified_collection_addresses: Option<Vec<String>>,
    pub first_verified_creators: Option<Vec<String>>,
    pub limit: Option<usize>,
    pub pagination_token: Option<String>,
}
impl MintListRequestConfig {
    pub fn generate_request_body(self) -> ClientResult<serde_json::Value> {
        if self.verified_collection_addresses.is_some() && self.first_verified_creators.is_some() {
            return single_verified_args_error();
        }

        if self.verified_collection_addresses.is_some() {
            Ok(json!({
                "query" : {
                    "verifiedCollectionAddresses": self.verified_collection_addresses.unwrap(),
                },
                "options": {
                    "limit": self.limit,
                    "paginationToken": self.pagination_token
                }
            }))
        } else {
            Ok(json!({
                "query": {
                    "firstVerifiedCreators": self.first_verified_creators.unwrap(),
                },
                "options": {
                    "limit": self.limit,
                    "paginationToken": self.pagination_token
                }
            }))
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveListingsResponse {
    pub result: Vec<ListingResult>,
    pub pagination_token: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ListingResult {
    pub mint: String,
    pub name: String,
    pub first_verified_creator: String,
    pub verified_collection_address: String,
    pub active_listings: Vec<ActiveListing>,
}

/// Request parameters for the `https://api.helius.xyz/v1/active-listings` endpoint. The API only accepts one of verified_collection_addresses or first_verified_creators, not both.
#[derive(Debug, Default)]
pub struct ActiveListingsRequestConfig {
    pub marketplaces: Vec<String>,
    pub verified_collection_addresses: Option<Vec<String>>,
    pub first_verified_creators: Option<Vec<String>>,
    pub limit: Option<usize>,
    pub pagination_token: Option<String>,
}
impl ActiveListingsRequestConfig {
    pub fn generate_request_body(self) -> ClientResult<serde_json::Value> {
        if self.verified_collection_addresses.is_some() && self.first_verified_creators.is_some() {
            return single_verified_args_error();
        }

        if self.verified_collection_addresses.is_some() {
            Ok(json!({
                "query" : {
                    "marketplaces": self.marketplaces,
                    "verifiedCollectionAddresses": self.verified_collection_addresses.unwrap(),
                },
                "options": {
                    "limit": self.limit,
                    "paginationToken": self.pagination_token
                }
            }))
        } else {
            Ok(json!({
                "query": {
                    "marketplaces": self.marketplaces,
                    "firstVerifiedCreators": self.first_verified_creators.unwrap(),
                },
                "options": {
                    "limit": self.limit,
                    "paginationToken": self.pagination_token
                }
            }))
        }
    }
}

fn single_verified_args_error<T>() -> ClientResult<T> {
    Err(ClientError::from(ClientErrorKind::Custom(
        "API only accepts one of first_verified_creators or verified_collection_addresses"
            .to_string(),
    )))
}
