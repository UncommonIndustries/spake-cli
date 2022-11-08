use reqwest::blocking::Client;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::params::API_ROUTE;
use url::{ParseError, Url};

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

fn build_api_endpoint(host: String) -> Result<Url, ParseError> {
    let base = Url::parse(&host).expect("Invalid host");
    let joined = base.join(API_ROUTE)?;
    Ok(joined)
}

#[derive(Debug, Clone)]
pub struct TranslationError;

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO expand this to give better feedback to users.
        write!(f, "Error translating string")
    }
}

pub fn translate_string(
    input: TranslationRequest,
    host: String,
) -> Result<TranslationResponse, TranslationError> {
    let client = Client::new();
    let fqdn = build_api_endpoint(host);
    let fqdn = match fqdn {
        Ok(fqdn) => fqdn,
        Err(error) => return Err(TranslationError),
    };

    let request = client.post(fqdn).json(&input);

    let res = match request.send() {
        Ok(res) => res,
        Err(error) => return Err(TranslationError),
    };

    let response: TranslationResponse = match res.json() {
        Ok(response) => response,
        Err(error) => return Err(TranslationError),
    };

    Ok(response)
}
