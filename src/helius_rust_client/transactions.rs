use super::helius_rust_client::{HeliusClient, API_URL_V0};
use reqwest::Client;
use serde::Deserialize;
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};
use solana_program::{clock::UnixTimestamp, slot_history::Slot};
use solana_sdk::commitment_config::CommitmentLevel;
use solana_transaction_status::{UiTransaction, UiTransactionStatusMeta};

use std::collections::HashMap;

use crate::models::{
    addresses::{MintListResponse, NftResponse, TokenBalancesResponse},
    nft::{NftEvent, NftMetadata},
    structs::TokenMetadata,
    transactions::{GetRawTransactionsRequestConfig, MintListRequestConfig, RequestConfig},
};

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
    /// Returns raw transaction history for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/raw-transactions`.
    /// * `config` - The [`RequestConfig`](crate::models::transactions::RequestConfig).
    pub async fn get_transactions(
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

    // /// Returns raw transaction information for the given transaction hashes. Calls `https://api.helius.xyz/v0/raw-transactions`.
    // /// * `transaction_hashes` - The transaction hashes as Strings.
    // /// * `commitment` - an Option containing the [`CommitmentLevel`]. Default is finalized.
    // pub async fn get_transactions_by_hash(
    //     &self,
    //     transaction_hashes: Vec<String>,
    //     commitment: Option<CommitmentLevel>
    // ) -> ClientResult<Vec<HeliusTxn>> {
    //     let mut request_url = format!(
    //         "{}/raw-transactions/?api-key={}",
    //         API_URL_V0,
    //         self.api_key,
    //     );

    //     match commitment {
    //         Some(CommitmentLevel::Confirmed) => {
    //             request_url.push_str("&commitment=confirmed");
    //         }
    //         Some(CommitmentLevel::Finalized) => {
    //             request_url.push_str("&commitment=finalized");
    //         },
    //         None => {
    //             request_url.push_str("&commitment=finalized");
    //         }
    //         _ => {
    //             return Err(ClientError::from(ClientErrorKind::Custom(
    //                 "Only Confirmed and Finalized commitments are supported by this API"
    //                     .to_string(),
    //             )));
    //         }
    //     }
    //     let mut body = HashMap::new();
    //     body.insert("transactions", transaction_hashes);

    //     let res: serde_json::Value = self
    //         .http_client
    //         .post(request_url)
    //         .header("accept", "application/json")
    //         .header("Content-Type", "application/json")
    //         .json(&body)
    //         .send()
    //         .await?
    //         .json()
    //         .await?;

    //     println!("{:?}", res);

    //     Ok(vec![])
    // }

    // // TODO

    // /// Returns enriched transaction history for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/transactions`.
    // /// * `address` - The address that you want transactions for.
    // pub async fn get_enriched_transactions(
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

    // /// Returns enriched transaction information for the given transaction hashes. Calls `https://api.helius.xyz/v0/transactions`.
    // /// * `transaction_hashes` - The transaction hashes as Strings.
    // /// * `commitment` - an Option containing the [`CommitmentLevel`]. Default is finalized.
    // pub async fn get_enriched_transactions_by_hash(
    //     &self,
    //     transaction_hashes: Vec<String>,
    //     commitment: Option<CommitmentLevel>
    // ) -> ClientResult<Vec<HeliusTxn>> {
    //     let mut request_url = format!(
    //         "{}/raw-transactions/?api-key={}",
    //         API_URL_V0,
    //         self.api_key,
    //     );

    //     match commitment {
    //         Some(CommitmentLevel::Confirmed) => {
    //             request_url.push_str("&commitment=confirmed");
    //         }
    //         Some(CommitmentLevel::Finalized) => {
    //             request_url.push_str("&commitment=finalized");
    //         },
    //         None => {
    //             request_url.push_str("&commitment=finalized");
    //         }
    //         _ => {
    //             return Err(ClientError::from(ClientErrorKind::Custom(
    //                 "Only Confirmed and Finalized commitments are supported by this API"
    //                     .to_string(),
    //             )));
    //         }
    //     }
    //     let mut body = HashMap::new();
    //     body.insert("transactions", transaction_hashes);

    //     let res: serde_json::Value = self
    //         .http_client
    //         .post(request_url)
    //         .header("accept", "application/json")
    //         .header("Content-Type", "application/json")
    //         .json(&body)
    //         .send()
    //         .await?
    //         .json()
    //         .await?;

    //     println!("{:?}", res);

    //     Ok(vec![])
    // }
}
