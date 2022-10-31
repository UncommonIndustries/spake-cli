use reqwest::blocking::Client;

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
    pub from_language: String, // ValidTargetLanguages,
    // The translated text.
    pub to_language: String, //ValidSourceLanguages,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslationResponse {
    // The translated text.
    text: String,
}

pub fn translate_string(input: TranslationRequest) {
    let client = Client::new();
    println! {"{:?}", input};
    let request = client
        .post("http://localhost:8000/api/v1/translate")
        .json(&input);
    println!("request : {:?}", request);
    let res = request.send().unwrap();
    let jsonresponse = res.json::<TranslationResponse>();
    println!("jsonresponse : {:?}", jsonresponse);
}
