use crate::models::transactions::TransactionType;

use super::helius_rust_client::{HeliusClient, API_URL_V0};
use reqwest::{Error as ReqwestError, Response, StatusCode};
use serde::{Deserialize, Serialize};
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    #[serde(rename = "webhookID")]
    pub webhook_id: String,
    pub wallet: String,
    #[serde(rename = "webhookURL")]
    pub webhook_url: String,
    pub transaction_types: Vec<TransactionType>,
    pub account_addresses: Vec<String>,
    pub webhook_type: WebhookType,
    pub auth_header: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookRequest {
    #[serde(rename = "webhookURL")]
    pub webhook_url: String,
    pub transaction_types: Vec<TransactionType>,
    pub account_addresses: Vec<String>,
    pub webhook_type: WebhookType,
    pub auth_header: String,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum WebhookType {
    enhanced,
    enhancedDevnet,
    raw,
    rawDevnet,
    discord,
    discordDevnet,
}

pub async fn parse_response<T: for<'a> Deserialize<'a>>(
    response: Result<Response, ReqwestError>,
) -> ClientResult<T> {
    match response {
        Ok(res) => {
            let payload = res.json().await; // could be `Error` or `Response` but only parses to `Response`
            match payload {
                Ok(j) => Ok(j),
                Err(e) => Err(ClientError::from(ClientErrorKind::Reqwest(e))),
            }
        }
        Err(e) => Err(ClientError::from(ClientErrorKind::Reqwest(e))),
    }
}

impl HeliusClient {
    /// Creates a webhook. POST request to `https://api.helius.xyz/v0/webhooks`.
    /// * `webhook_request` - The [`CreateWebhookRequest`](CreateWebhookRequest).
    pub async fn create_webhook(
        &self,
        webhook_request: CreateWebhookRequest,
    ) -> ClientResult<Webhook> {
        let request_url = format!("{}/webhooks/?api-key={}", API_URL_V0, self.api_key);

        let response = self
            .http_client
            .post(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&webhook_request)
            .send()
            .await;

        parse_response(response).await
    }

    /// Returns all webhooks for the API key used to create the [`HeliusClient`](super::helius_rust_client::HeliusClient). GET request to `https://api.helius.xyz/v0/webhooks`.
    pub async fn get_webhooks(&self) -> ClientResult<Vec<Webhook>> {
        let request_url = format!("{}/webhooks?api-key={}", API_URL_V0, self.api_key);

        let response = self
            .http_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await;

        parse_response(response).await
    }

    /// Returns a single webhook. GET request to `https://api.helius.xyz/v0/webhooks/{webhook_id}`.
    /// * `webhook_id` - The webhook that you want to fetch.
    pub async fn get_webhook(&self, webhook_id: String) -> ClientResult<Webhook> {
        let request_url = format!(
            "{}/webhooks/{}?api-key={}",
            API_URL_V0, webhook_id, self.api_key
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

    /// Edits a webhook. PUT request to `https://api.helius.xyz/v0/webhooks/{webhook_id}`.
    /// * `webhook_id` - The webhook that you want to edit.
    pub async fn edit_webhook(
        &self,
        webhook_id: String,
        new_webhook: CreateWebhookRequest,
    ) -> ClientResult<Webhook> {
        let request_url = format!(
            "{}/webhooks/{}?api-key={}",
            API_URL_V0, webhook_id, self.api_key
        );

        let response = self
            .http_client
            .put(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&new_webhook)
            .send()
            .await;

        parse_response(response).await
    }

    /// Deletes a webhook. DELETE request to `https://api.helius.xyz/v0/webhooks/{webhook_id}`.
    /// * `webhook_id` - The webhook that you want to delete.
    pub async fn delete_webhook(&self, webhook_id: String) -> ClientResult<()> {
        let request_url = format!(
            "{}/webhooks/{}?api-key={}",
            API_URL_V0, webhook_id, self.api_key
        );

        let response = self
            .http_client
            .delete(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(())
                } else {
                    Err(ClientError::from(ClientErrorKind::Custom(format!(
                        "Request failed with status code: {}",
                        res.status()
                    ))))
                }
            }
            Err(e) => Err(ClientError::from(ClientErrorKind::Reqwest(e))),
        }
    }
}
