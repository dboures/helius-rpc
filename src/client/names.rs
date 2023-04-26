use super::{
    init::{HeliusClient, API_URL_V0},
    parse_response,
};
use serde::Deserialize;
use solana_client::client_error::Result as ClientResult;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct DomainNamesResponse {
    domain_names: Vec<String>,
}

impl HeliusClient {
    /// Returns the Solana Naming Service name for a given address. GET request to `https://api.helius.xyz/v0/addresses/{address}/names`.
    /// * `address` - The address that you want names for.
    pub async fn get_naming_service_names(&self, address: String) -> ClientResult<Vec<String>> {
        let request_url = format!(
            "{}/addresses/{}/names?api-key={}",
            API_URL_V0, address, self.api_key
        );

        let response = self
            .http_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await;

        let response: DomainNamesResponse = parse_response(response).await?;
        Ok(response.domain_names)
    }
}
