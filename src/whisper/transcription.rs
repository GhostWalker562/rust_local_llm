use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct WhisperListenBody {
    pub model: String,
    pub duration: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WhisperTranscriptionResponse {
    pub text: String,
    pub language: String,
}
