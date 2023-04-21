use reqwest::{Response, Error as ReqwestError};
use serde::Deserialize;
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};

pub mod helius_rust_client;
pub mod transactions;
pub mod webhooks;
pub mod tokens;
pub mod names;


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

pub fn api_commitment_error<T>() -> ClientResult<T> {
    Err(ClientError::from(ClientErrorKind::Custom(
        "Only Confirmed and Finalized commitments are supported by this API"
            .to_string(),
    )))
}