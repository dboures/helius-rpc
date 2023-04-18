use reqwest::Client as RestClient;
use serde_json::Value;
use solana_client::client_error::Result as ClientResult;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::genesis_config::ClusterType;
use std::collections::HashMap;

use crate::helius_rpc::structs::{NftMetadata, TokenMetadata};

pub const API_URL_V0: &str = "https://api.helius.xyz/v0";
pub const API_URL_V1: &str = "https://api.helius.xyz/v1";
pub const MAINNET_RPC_URL: &str = "https://rpc.helius.xyz/?api-key=";
pub const DEVNET_RPC_URL: &str = "https://rpc-devnet.helius.xyz/?api-key=";

pub struct HeliusRpcClient {
    pub rpc_client: RpcClient,
    pub cluster: ClusterType,
    pub(crate) api_key: String,
    pub(crate) rest_client: RestClient,
}
impl HeliusRpcClient {
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
            .rest_client
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
            .rest_client
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
