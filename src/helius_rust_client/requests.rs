use super::helius_rust_client::{HeliusClient, API_URL_V0};
use serde::Deserialize;
use solana_client::client_error::Result as ClientResult;
use solana_program::{clock::UnixTimestamp, slot_history::Slot};
use solana_transaction_status::{UiTransaction, UiTransactionStatusMeta};

use std::collections::HashMap;

use crate::models::{nft::{NftMetadata, NftEvent}, addresses::{TokenBalancesResponse, NftResponse, DomainNames}, structs::TokenMetadata, transactions::{GetRawTransactionsRequestConfig, RequestConfig}};

use super::{
    helius_rust_client::{ API_URL_V1},
};

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusTxn {
    pub slot: Slot,
    pub block_time: Option<UnixTimestamp>,
    pub transaction: UiTransaction,
    pub meta: Option<UiTransactionStatusMeta>,
}

impl HeliusClient {
    /// Returns the Solana Naming Service name for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/names`.
    /// * `address` - The addresses that you want names for.
    pub async fn get_naming_service_names(&self, address: String) -> ClientResult<Vec<String>> {
        let request_url = format!(
            "{}/addresses/{}/names?api-key={}",
            API_URL_V0, address, self.api_key
        );

        let res: DomainNames = self
            .http_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json()
            .await?;
        Ok(res.domain_names)
    }

    /// Returns the native balance and token balances for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/balances`.
    /// * `address` - The addresses that you want token balances for.
    pub async fn get_token_balances(&self, address: String) -> ClientResult<TokenBalancesResponse> {
        let request_url = format!(
            "{}/addresses/{}/balances?api-key={}",
            API_URL_V0, address, self.api_key
        );

        let res: TokenBalancesResponse = self
            .http_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
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

        let res: NftResponse = self
            .http_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
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

    pub async fn get_nft_events(&self, config: RequestConfig) -> ClientResult<Vec<NftEvent>> {
        let query = config.generate_query_parameters(self.api_key.clone())?;
        let request_url = format!(
            "{}/addresses/{}/nft-events?",
            API_URL_V0,
            config.address.to_string(),
        );

        let res: Vec<NftEvent> = self
            .http_client
            .get(request_url)
            .query(&query)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
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

    /// Returns raw transaction history for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/raw-transactions`.
    /// * `address` - The address that you want transactions for.
    pub async fn get_raw_transactions(
        &self,
        config: GetRawTransactionsRequestConfig,
    ) -> ClientResult<Vec<HeliusTxn>> {
        let query = config.generate_query_parameters(self.api_key.clone())?;
        let request_url = format!(
            "{}/addresses/{}/raw-transactions?",
            API_URL_V0,
            config.address.to_string(),
        );

        let res: Vec<HeliusTxn> = self
            .http_client
            .get(request_url)
            .query(&query)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }

    // // TODO

    // /// Returns enriched transaction history for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/transactions`.
    // /// * `address` - The address that you want transactions for.
    // pub async fn get_transactions(
    //     &self,
    //     config: GetTransactionsRequestConfig,
    // ) -> ClientResult<Vec<HeliusTxn>> {
    //         let query = config.generate_query_parameters(self.api_key.clone())?;
    //         let request_url = format!(
    //             "{}/addresses/{}/transactions?",
    //             API_URL_V0, config.address.to_string(),
    //         );

    //         let res: serde_json::Value = self
    //             .http_client
    //             .get(request_url)
    //             .query(&query)
    //             .send()
    //             .await?
    //             .json()
    //             .await?;

    //         println!("{:?}", res);
    //         Ok(vec![])
    // }

}