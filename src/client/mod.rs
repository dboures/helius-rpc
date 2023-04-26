use reqwest::{Error as ReqwestError, Response};
use serde::Deserialize;
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};

pub mod init;
pub mod names;
pub mod tokens;
pub mod transactions;
pub mod webhooks;

pub async fn parse_response<T: for<'a> Deserialize<'a>>(
    response: Result<Response, ReqwestError>,
) -> ClientResult<T> {
    match response {
        Ok(res) => {
            let payload = res.json().await;
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
        "Only Confirmed and Finalized commitments are supported by this API".to_string(),
    )))
}
