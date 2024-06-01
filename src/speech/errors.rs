#[derive(Debug)]
pub struct SpeechError {
    pub code: i32,
    pub message: String,
}

impl SpeechError {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }

    pub fn tts_error() -> Self {
        Self {
            code: 1,
            message: "An error occurred while generating speech.".to_string(),
        }
    }

    pub fn playback_error() -> Self {
        Self {
            code: 2,
            message: "An error occurred while playing the audio.".to_string(),
        }
    }
}
