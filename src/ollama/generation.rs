use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct OllamaGenerationBody {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OllamaGenerationResponse {
    pub model: String,
    pub response: String,
}
