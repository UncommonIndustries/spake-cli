use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslationResponse {
    // The translated text.
    pub text: String,
    pub quality_score: f64,
}
