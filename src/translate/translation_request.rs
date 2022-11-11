use serde::{Deserialize, Serialize};

use super::models::{ValidSourceLanguages, ValidTargetLanguages};

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
