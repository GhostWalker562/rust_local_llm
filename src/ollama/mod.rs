pub mod errors;
pub mod generation;

use errors::OllamaError;
use url::Url;

use generation::{OllamaGenerationBody, OllamaGenerationResponse};

#[derive(Debug, Clone)]
pub struct Ollama {
    url: Url,
    reqwest_client: reqwest::Client,
}

impl Ollama {
    /// Create a new instance of the Ollama client that can be used to generate text using an AI model.
    ///
    /// # Examples
    /// ```
    /// let ollama = Ollama::new();
    /// let generation_body = OllamaGenerationBody {
    ///     model: String::from("gemma"),
    ///     prompt: String::from("Hello, what's 1 + 1?"),
    ///     stream: false
    /// };
    /// let response = ollama.generate(generation_body).await.unwrap();
    /// println!("{}", response.text);
    /// ```
    pub fn new() -> Self {
        Self {
            url: Url::parse("http://127.0.0.1:11434").unwrap(),
            reqwest_client: reqwest::Client::new(),
        }
    }

    /// Generate text using an AI model interfacing with Ollama.
    ///
    /// # Examples
    ///
    /// ```
    /// let ollama = Ollama::new();
    /// let generation_body = OllamaGenerationBody {
    ///     model: String::from("gemma"),
    ///     prompt: String::from("Hello, what's 1 + 1?"),
    ///     stream: false
    /// };
    /// let response = ollama.generate(generation_body).await.unwrap();
    /// println!("{}", response.text);
    /// ```
    pub async fn generate(
        &self,
        body: OllamaGenerationBody,
    ) -> Result<OllamaGenerationResponse, OllamaError> {
        let endpoint = format!("{}api/generate", self.url.as_str());
        let request_body = serde_json::to_string(&body).unwrap();

        let res = self
            .reqwest_client
            .post(endpoint)
            .body(request_body)
            .send()
            .await
            .map_err(|_| OllamaError::generation_error())?;

        if !res.status().is_success() {
            return Err(OllamaError::generation_error());
        }

        let data = res.text().await.unwrap();
        Ok(serde_json::from_str::<OllamaGenerationResponse>(data.as_str()).unwrap())
    }
}
