#[derive(Debug)]
pub struct WhisperError {
    pub code: i32,
    pub message: String,
}

impl WhisperError {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }

    pub fn no_default_input_device() -> Self {
        Self {
            code: 1,
            message: "No default input device found.".to_string(),
        }
    }

    pub fn recording_error() -> Self {
        Self {
            code: 2,
            message: "An error occurred while recording.".to_string(),
        }
    }

    pub fn transcription_error() -> Self {
        Self {
            code: 3,
            message: "An error occurred while transcribing.".to_string(),
        }
    }
}
