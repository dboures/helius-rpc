use serde::Deserialize;

use super::transactions::{NftEventType, TokenStandard};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveListing {
    pub transaction_signature: String,
    pub marketplace: String,
    pub amount: i32,
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


#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NftEvent {
    pub amount: i128, // Sometimes has negatives, although it should be lamports , TODO: write a lamport deserializer??
    pub fee: u64,
    pub fee_payer: String,
    pub signature: String,
    pub slot: u64,
    pub timestamp: u64,
    #[serde(rename = "type")]
    pub sale_type: NftEventType,
    pub buyer: String,
    pub seller: String,
    pub staker: String,
    pub nfts: Vec<NftToken>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NftToken {
    pub mint: String,
    pub token_standard: TokenStandard
}