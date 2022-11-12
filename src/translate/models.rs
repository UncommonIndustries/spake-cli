use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
