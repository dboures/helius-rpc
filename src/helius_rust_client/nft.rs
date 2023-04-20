use serde_json::Value;
use solana_client::client_error::Result as ClientResult;
use std::collections::HashMap;

use crate::models::nft::NftMetadata;

use super::{
    helius_rust_client::{HeliusClient, API_URL_V0, API_URL_V1},
    structs::TokenMetadata,
};

impl HeliusClient {
    // TODO: helius specific things

    // pub async fn get_mint_list(&self) {
    //     let client = reqwest::Client::new();
    //     let request_url = format!("{}/mintlist?api-key={}", API_URL_V1, self.api_key);
    //     // let query = [("verifiedCollectionAddresses", vec!["6XxjKYFbcndh2gDcsUrmZgVEsoDxXMnfsaGY6fpTJzNr"])];

    //     let first_verified_creators = vec!["A4FM6h8T5Fmh9z2g3fKUrKfZn6BNFEgByR8QGpdbQhk1"];
    //     let query = format!(
    //         r#"{{"query": {{"firstVerifiedCreators": {:#?}}}}}"#,
    //         first_verified_creators
    //     );

    //     let res = client.post(request_url).json(&query).send().await.unwrap();
    // }

    /// Returns token metadata (whether NFT or Fungible) for the given token mint addresses. Calls `https://api.helius.xyz/v0/tokens/metadata`.
    /// * `token_mints` - The token mint addresses that you want metadata for.
    pub async fn get_tokens_metadata(
        &self,
        token_mints: Vec<String>,
    ) -> ClientResult<Vec<TokenMetadata>> {
        let request_url = format!("{}/tokens/metadata?api-key={}", API_URL_V0, self.api_key);
        let mut body = HashMap::new();
        body.insert("mintAccounts", token_mints);

        let res: Vec<TokenMetadata> = self
            .http_client
            .post(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }

    /// Returns NFT metadata for the given token mint addresses. Calls `https://api.helius.xyz/v1/nfts`.
    /// * `token_mints` - The nft mint addresses that you want metadata for.
    pub async fn get_nfts_metadata(
        &self,
        token_mints: Vec<String>,
    ) -> ClientResult<Vec<NftMetadata>> {
        let request_url = format!("{}/nfts?api-key={}", API_URL_V1, self.api_key);
        let mut body = HashMap::new();
        body.insert("mints", token_mints);

        let res: Vec<NftMetadata> = self
            .http_client
            .post(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }
}
