use crate::models::{
    enriched_transaction::RequestConfig,
    nft::{
        ActiveListingsRequestConfig, ActiveListingsResponse, MintListRequestConfig,
        MintListResponse, NftEvent, NftMetadata, NftResponse, TokenBalancesResponse,
    },
    structs::TokenMetadata,
};

use super::{
    helius_rust_client::{HeliusClient, API_URL_V0},
    parse_response,
};
use reqwest::Client;
use serde::Deserialize;
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};
use solana_program::{clock::UnixTimestamp, slot_history::Slot};
use solana_sdk::commitment_config::CommitmentLevel;
use solana_transaction_status::{UiTransaction, UiTransactionStatusMeta};

use std::collections::HashMap;

use super::helius_rust_client::API_URL_V1;

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusTxn {
    pub slot: Slot,
    pub block_time: Option<UnixTimestamp>,
    pub transaction: UiTransaction,
    pub meta: Option<UiTransactionStatusMeta>,
}

impl HeliusClient {
    /// Returns the native balance and token balances for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/balances`.
    /// * `address` - The addresses that you want token balances for.
    pub async fn get_token_balances(&self, address: String) -> ClientResult<TokenBalancesResponse> {
        let request_url = format!(
            "{}/addresses/{}/balances?api-key={}",
            API_URL_V0, address, self.api_key
        );

        let response = self
            .http_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await;

        parse_response(response).await
    }

    /// Returns the NFTs held for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/nfts`.
    /// * `address` - The addresses that you want nfts for.
    pub async fn get_nfts(
        &self,
        address: String,
        page_number: Option<usize>,
    ) -> ClientResult<NftResponse> {
        let mut request_url = format!(
            "{}/addresses/{}/nfts?api-key={}",
            API_URL_V0, address, self.api_key
        );

        if page_number.is_some() {
            request_url = format!("{}&pageNumber={}", request_url, page_number.unwrap());
        }

        let response = self
            .http_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await;

        parse_response(response).await
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

        let response = self
            .http_client
            .post(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;

        parse_response(response).await
    }

    // /// Returns all NFT related events associated with the given address. POST request to `https://api.helius.xyz/v1/nft-events`.
    // /// * `config` - The [`RequestConfig`](crate::models::enriched_transaction::RequestConfig).
    // pub async fn get_nft_events_for_address(&self, config: RequestConfig) -> ClientResult<Vec<NftEvent>> {
    //     let query = config.generate_query_parameters(self.api_key.clone())?;
    //     let request_url = format!(
    //         "{}/addresses/{}/nft-events?",
    //         API_URL_V0,
    //         config.address.to_string(),
    //     );

    //     let res: Vec<NftEvent> = self
    //         .http_client
    //         .get(request_url)
    //         .query(&query)
    //         .send()
    //         .await?
    //         .json()
    //         .await?;

    //     Ok(res)
    // }

    /// Returns all NFT related events associated with the given address. Calls `https://api.helius.xyz/v1/addresses/{address}/nft-events`.
    /// * `config` - The [`RequestConfig`](crate::models::enriched_transaction::RequestConfig).
    pub async fn get_nft_events(&self, config: RequestConfig) -> ClientResult<Vec<NftEvent>> {
        let query = config.generate_query_parameters(self.api_key.clone())?;
        let request_url = format!(
            "{}/addresses/{}/nft-events?",
            API_URL_V0,
            config.address.to_string(),
        );

        let response = self.http_client.get(request_url).query(&query).send().await;

        parse_response(response).await
    }

    /// Query for active NFT listings. POST request to `https://api.helius.xyz/v1/active-listings`.
    /// * `config` - The [`ActiveListingsRequestConfig`](crate::models::nft::ActiveListingsRequestConfig).
    pub async fn get_active_nft_listings(
        &self,
        config: ActiveListingsRequestConfig,
    ) -> ClientResult<ActiveListingsResponse> {
        let body = config.generate_request_body()?;
        let request_url = format!("{}/active-listings?api-key={}", API_URL_V1, self.api_key);

        let response = self
            .http_client
            .post(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;

        parse_response(response).await
    }

    /// Returns a list of mint accounts for a given NFT collection. POST request to `https://api.helius.xyz/v1/mintlist`.
    /// * `config` - The [`MintListRequestConfig`](crate::models::nft::MintListRequestConfig).
    pub async fn get_mint_list(
        &self,
        config: MintListRequestConfig,
    ) -> ClientResult<MintListResponse> {
        let request_url = format!("{}/mintlist?api-key={}", API_URL_V1, self.api_key);

        let body = config.generate_request_body()?;
        let response = self
            .http_client
            .post(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;

        parse_response(response).await
    }

    /// Returns token metadata (whether NFT or Fungible) for the given token mint addresses. Calls `https://api.helius.xyz/v0/tokens/metadata`.
    /// * `token_mints` - The token mint addresses that you want metadata for.
    pub async fn get_tokens_metadata(
        &self,
        token_mints: Vec<String>,
    ) -> ClientResult<Vec<TokenMetadata>> {
        let request_url = format!("{}/tokens/metadata?api-key={}", API_URL_V0, self.api_key);
        let mut body = HashMap::new();
        body.insert("mintAccounts", token_mints);

        let response = self
            .http_client
            .post(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;

        parse_response(response).await
    }
}
