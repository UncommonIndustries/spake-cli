use reqwest::blocking::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::params::API_ROUTE;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ValidTargetLanguages {
    es,
    fr,
    de,
    it,
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ValidSourceLanguages {
    en,
}

impl FromStr for ValidTargetLanguages {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "es" => Ok(ValidTargetLanguages::es),
            "fr" => Ok(ValidTargetLanguages::fr),
            "de" => Ok(ValidTargetLanguages::de),
            "it" => Ok(ValidTargetLanguages::it),
            _ => Err(format!("{} is not a valid target language", s)),
        }
    }
}

// translate.rs is a service to translate the input text into the target language.
#[derive(Serialize, Deserialize, Debug)]
pub struct TranslationRequest {
    // The input text to translate.
    pub text: String,
    // The target language to translate the input text.
    pub from_language: ValidSourceLanguages,
    // The translated text.
    pub to_language: ValidTargetLanguages,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslationResponse {
    // The translated text.
    pub text: String,
}

fn build_api_endpoint(host: String) -> String {
    format!("{}/{}", host, API_ROUTE,)
}

pub fn translate_string(
    input: TranslationRequest,
    host: String,
) -> Result<TranslationResponse, Error> {
    let client = Client::new();
    let fqdn = build_api_endpoint(host);
    let request = client.post(fqdn).json(&input);

    let res = request.send()?;
    let response: TranslationResponse = res.json()?;
    Ok(response)
}
