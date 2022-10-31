use reqwest::blocking::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ValidTargetLanguages {
    es,
    fr,
    de,
    it,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ValidSourceLanguages {
    en,
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

pub fn translate_string(input: TranslationRequest) -> Result<TranslationResponse, Error> {
    let client = Client::new();

    let request = client
        .post("http://localhost:8000/api/v1/translate")
        .json(&input);

    let res = request.send()?;
    let response: TranslationResponse = res.json()?;
    Ok(response)
}
