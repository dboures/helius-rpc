use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenMetadata {
    pub mint: String,
    pub on_chain_data: Option<OnChainMetadata>,
    pub off_chain_data: Option<OffChainMetadata>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OnChainMetadata {
    pub key: String,
    pub mint: String,
    pub update_authority: String,
    pub data: OnChainData,
    pub token_standard: String,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: i32,
    pub collection: Option<Collection>,
    pub collection_details: Option<CollectionDetails>,
    pub uses: Uses,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename = "data")]
#[serde(rename_all = "camelCase")]
pub struct OnChainData {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: i32,
    pub creators: Option<Vec<Creator>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub address: String,
    pub share: u64,
    pub verified: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub key: String,
    pub verified: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDetails {
    pub size: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Uses {
    pub use_method: String,
    pub remaining: i32,
    pub total: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OffChainMetadata {
    pub name: String,
    pub symbol: String,
    pub attributes: Vec<Attribute>,
    pub seller_fee_basis_points: i32,
    pub image: String,
    pub properties: Properties,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub category: String,
    pub files: Vec<File>,
    pub creators: Vec<Creator>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub uri: String,
    #[serde(rename = "type")]
    pub file_type: String,
}
