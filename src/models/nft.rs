use serde::Deserialize;

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
