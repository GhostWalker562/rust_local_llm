# CSS498 - Rust Local LLM Project
This is a project for the CSS498 course at the University of Washington. The project is a Rust implementation of interfaces to local LLMs that can provide an end-to-end speech-to-text system.

## Pre-requisites
You must have the following programs installed on your machine to run the project:
- [Ollama](https://ollama.com/)
- [TTS](https://github.com/coqui-ai/TTS)
- [Whisper](https://github.com/openai/whisper)
- [Rust](https://www.rust-lang.org/)

## Usage
To use the project, you must have the pre-requisites installed on your machine. You can then run the project with the following command:

```bash
cargo run
```

## Architecture

The project is divided into the following modules:

- `ollama`: This module contains the interface to the Ollama LLM.
- `tts`: This module contains the interface to the TTS LLM.
- `whisper`: This module contains the interface to the Whisper LLM.

### Visual Representation

<image src="./assets/architecture.png" alt="Architecture Diagram"/>