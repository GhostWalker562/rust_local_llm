pub mod ollama;
pub mod speech;
pub mod whisper;

use ollama::{generation::OllamaGenerationBody, Ollama};
use speech::Speech;
use whisper::{transcription::WhisperListenBody, Whisper};

#[tokio::main]
async fn main() {
    let whisper = Whisper::new(); // Speech to text
    let ollama = Ollama::new(); // Text completion
    let speech = Speech::new(); // Text to speech

    // Listen for user input.
    println!(
        "Listening with {:?}, Say something!",
        whisper
            .get_default_input_device()
            .unwrap_or(String::from(""))
    );
    let listen_body = WhisperListenBody {
        duration: 5,
        model: String::from("small"),
    };
    let user_response = whisper.listen_and_transcribe(listen_body).unwrap();
    println!("You said: {}\n", user_response.text);

    // Generate a response.
    println!("Generating response...");
    let generation_body = OllamaGenerationBody {
        model: String::from("gemma"),
        prompt: user_response.text,
        stream: false,
    };
    let generation_response = ollama.generate(generation_body).await.unwrap();
    println!("Response: {}", generation_response.response);

    // Speak the generated response.
    speech
        .speak(&generation_response.response, 1.0)
        .unwrap_or_else(|e| println!("Error: {:?}", e));
}
