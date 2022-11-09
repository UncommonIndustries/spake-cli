use reqwest::blocking::Client;

use serde::{Deserialize, Serialize};
use std::error;

use std::str::FromStr;

use crate::params::API_ROUTE;
use url::{ParseError, Url};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]

pub enum ValidTargetLanguages {
    en,
    es,
    fr,
    de,
    it,
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum ValidSourceLanguages {
    en,
    es,
    fr,
    de,
    it,
}
impl FromStr for ValidSourceLanguages {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(ValidSourceLanguages::en),
            "es" => Ok(ValidSourceLanguages::es),
            "fr" => Ok(ValidSourceLanguages::fr),
            "de" => Ok(ValidSourceLanguages::de),
            "it" => Ok(ValidSourceLanguages::it),
            _ => Err(format!("{} is not a valid source language", s)),
        }
    }
}
impl FromStr for ValidTargetLanguages {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "es" => Ok(ValidTargetLanguages::es),
            "fr" => Ok(ValidTargetLanguages::fr),
            "de" => Ok(ValidTargetLanguages::de),
            "it" => Ok(ValidTargetLanguages::it),
            "en" => Ok(ValidTargetLanguages::en),
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

pub fn translate_string(
    input: TranslationRequest,
    host: String,
) -> Result<TranslationResponse, Box<dyn error::Error>> {
    let client = Client::new();
    let fqdn = build_api_endpoint(host);
    let fqdn = match fqdn {
        Ok(fqdn) => fqdn,
        Err(err) => return Err(err.into()),
    };

    let request = client.post(fqdn).json(&input);

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
