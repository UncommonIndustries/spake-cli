use reqwest::blocking::Client;

use serde::{Deserialize, Serialize};
use std::error;
use std::str::FromStr;

use crate::params::API_ROUTE;
use url::{ParseError, Url};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]

pub enum ValidTargetLanguages {
    En,
    Es,
    Fr,
    De,
    It,
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub enum ValidSourceLanguages {
    En,
    Es,
    Fr,
    De,
    It,
}
impl FromStr for ValidSourceLanguages {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(ValidSourceLanguages::En),
            "es" => Ok(ValidSourceLanguages::Es),
            "fr" => Ok(ValidSourceLanguages::Fr),
            "de" => Ok(ValidSourceLanguages::De),
            "it" => Ok(ValidSourceLanguages::It),
            _ => Err(format!("{} is not a valid source language", s)),
        }
    }
}
impl FromStr for ValidTargetLanguages {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "es" => Ok(ValidTargetLanguages::Es),
            "fr" => Ok(ValidTargetLanguages::Fr),
            "de" => Ok(ValidTargetLanguages::De),
            "it" => Ok(ValidTargetLanguages::It),
            "en" => Ok(ValidTargetLanguages::En),
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

    let response: TranslationResponse = match res.json() {
        Ok(response) => response,
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(err.into());
        }
    };

    Ok(response)
}
