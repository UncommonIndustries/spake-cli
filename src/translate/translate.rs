use reqwest::blocking::Client;

use std::error;

use super::translation_request::TranslationRequest;
use super::translation_response::TranslationResponse;

use crate::params::API_ROUTE;
use url::{ParseError, Url};

fn build_api_endpoint(host: String) -> Result<Url, ParseError> {
    let base = Url::parse(&host).expect("Invalid host");
    let joined = base.join(API_ROUTE)?;
    Ok(joined)
}

pub fn translate_string(
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

    let res = match request.send() {
        Ok(res) => res,
        Err(err) => return Err(err.into()),
    };

    match res.status() {
        reqwest::StatusCode::OK => (),
        _ => {
            println!("Error: {:?}", res.json::<serde_json::Value>());
            return Err("Error making request".into());
        }
    }

    let response: TranslationResponse = match res.json() {
        Ok(response) => response,
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(err.into());
        }
    };

    Ok(response)
}
