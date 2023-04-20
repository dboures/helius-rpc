use serde::Deserialize;
use solana_program::{clock::UnixTimestamp, slot_history::Slot};
use solana_transaction_status::UiTransaction;

use super::transactions::UiTransactionStatusMeta;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DomainNames {
    pub domain_names: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalancesResponse {
    pub native_balance: u64,
    pub tokens: Vec<TokenBalance>,
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

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusTxn {
    pub slot: Slot,
    pub block_time: Option<UnixTimestamp>,
    pub transaction: UiTransaction,
    pub meta: Option<UiTransactionStatusMeta>,
}