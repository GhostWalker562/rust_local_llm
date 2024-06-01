#[derive(Debug)]
pub struct OllamaError {
    pub code: i32,
    pub message: String,
}

impl OllamaError {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }

    pub fn generation_error() -> Self {
        Self {
            code: 1,
            message: "An error occurred while generating the text.".to_string(),
        }
    }
}
