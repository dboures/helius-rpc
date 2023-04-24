use super::{helius_rust_client::{HeliusClient, API_URL_V0}, parse_response};
use solana_client::client_error::{ Result as ClientResult};
use solana_sdk::commitment_config::CommitmentLevel;

use std::collections::HashMap;

use crate::{models::{
    raw_transaction::{GetRawTransactionsRequestConfig, RawTransaction}, enriched_transaction::{EnrichedTransaction, RequestConfig},
}, helius_rust_client::api_commitment_error};


impl HeliusClient {
    /// Returns raw transaction history for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/raw-transactions`.
    /// * `config` - The [`RequestConfig`](crate::models::transactions::RequestConfig).
    pub async fn get_transactions_for_address(
        &self,
        config: GetRawTransactionsRequestConfig,
    ) -> ClientResult<Vec<RawTransaction>> {
        let query = config.generate_query_parameters(self.api_key.clone())?;
        let request_url = format!(
            "{}/addresses/{}/raw-transactions?",
            API_URL_V0,
            config.address.to_string(),
        );

        let response = self
            .http_client
            .get(request_url)
            .query(&query)
            .send()
            .await;

        parse_response(response).await
    }

    /// Returns raw transaction information for the given transaction hashes. POST request to `https://api.helius.xyz/v0/raw-transactions`.
    /// * `transaction_hashes` - The transaction hashes as Strings.
    /// * `commitment` - an Option containing the [`CommitmentLevel`]. Default is finalized.
    pub async fn get_transactions_by_hash(
        &self,
        transaction_hashes: Vec<String>,
        commitment: Option<CommitmentLevel>
    ) -> ClientResult<Vec<RawTransaction>> {
        let request_url = format!(
            "{}/raw-transactions?api-key={}",
            API_URL_V0,
            self.api_key,
        );

        let request_url = attach_commitment(request_url, commitment)?;
        let mut body = HashMap::new();
        body.insert("transactions", transaction_hashes);

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

    /// Returns enriched transaction history for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/transactions`.
    /// * `address` - The address that you want transactions for.
    pub async fn get_enriched_transactions(
        &self,
        config: RequestConfig,
    ) -> ClientResult<Vec<EnrichedTransaction>> {
            let query = config.generate_query_parameters(self.api_key.clone())?;
            let request_url = format!(
                "{}/addresses/{}/transactions?",
                API_URL_V0, config.address.to_string(),
            );
            let request_url = attach_commitment(request_url, config.commitment)?;

            let response = self
                .http_client
                .get(request_url)
                .query(&query)
                .send()
                .await;

            parse_response(response).await
    }

    /// Returns enriched transaction information for the given transaction hashes. Calls `https://api.helius.xyz/v0/transactions`.
    /// * `transaction_hashes` - The transaction hashes as Strings.
    /// * `commitment` - an Option containing the [`CommitmentLevel`]. Default is finalized.
    pub async fn get_enriched_transactions_by_hash(
        &self,
        transaction_hashes: Vec<String>,
        commitment: Option<CommitmentLevel>
    ) -> ClientResult<Vec<EnrichedTransaction>> {
        let request_url = format!(
            "{}/transactions/?api-key={}",
            API_URL_V0,
            self.api_key,
        );

        let request_url = attach_commitment(request_url, commitment)?;

        let mut body = HashMap::new();
        body.insert("transactions", transaction_hashes);

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


fn attach_commitment(mut request_url: String, commitment: Option<CommitmentLevel>) -> ClientResult<String> {
        match commitment {
            Some(CommitmentLevel::Confirmed) => {
                request_url.push_str("&commitment=confirmed");
            }
            Some(CommitmentLevel::Finalized) => {
                request_url.push_str("&commitment=finalized");
            },
            None => {
                request_url.push_str("&commitment=finalized");
            }
            _ => {
                return api_commitment_error()
            }
        }
        Ok(request_url)
}