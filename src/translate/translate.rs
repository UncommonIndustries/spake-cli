use reqwest::Client;
use std::error;

use super::translation_request::TranslationRequest;
use super::translation_response::TranslationResponse;

use crate::params::API_ROUTE;
use url::{ParseError, Url};

pub(crate) fn build_api_endpoint(host: String) -> Result<Url, ParseError> {
    let base = Url::parse(&host)?;
    let joined = base.join(API_ROUTE)?;
    Ok(joined)
}

pub async fn translate_string(
    input: TranslationRequest,
    host: String,
    api_key: String,
) -> Result<TranslationResponse, Box<dyn error::Error>> {
    let client = Client::new();
    let fqdn = build_api_endpoint(host);
    let fqdn = match fqdn {
        Ok(fqdn) => fqdn,
        Err(err) => return Err(err.into()),
    };

    let request = client.post(fqdn).json(&input).header("X-API-KEY", api_key);

    let res = match request.send().await {
        Ok(res) => res,
        Err(err) => return Err(err.into()),
    };

    match res.status() {
        reqwest::StatusCode::OK => (),
        reqwest::StatusCode::BAD_REQUEST => {
            return Err("Bad request".into());
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            let message = res.json::<serde_json::Value>().await?;
            let details = message["detail"].to_string();
            let response_string = format!("Forbidden: {}", details);
            return Err(response_string.into());
        }
        reqwest::StatusCode::FORBIDDEN => {
            let message = res.json::<serde_json::Value>().await?;
            let details = message["detail"].to_string();
            let response_string = format!("Forbidden: {}", details);
            return Err(response_string.into());
        }
        reqwest::StatusCode::NOT_FOUND => {
            return Err("Not found".into());
        }
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
            return Err("Internal server error".into());
        }
        _ => {
            return Err("Error making request".into());
        }
    }

    let response: TranslationResponse = match res.json().await {
        Ok(response) => response,
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(err.into());
        }
    };

    Ok(response)
}
