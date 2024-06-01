pub mod errors;
pub mod transcription;

use std::{fs, path::Path};

use cmd_lib::run_fun;
use cpal::traits::{DeviceTrait, HostTrait};

use errors::WhisperError;
use transcription::{WhisperListenBody, WhisperTranscriptionResponse};

#[derive(Debug, Clone)]
pub struct Whisper {}

impl Whisper {
    /// Listening and transcription client using the OpenAI Whisper model.
    ///
    /// # Examples
    /// ```
    /// let whisper = Whisper::new();
    /// let listen_body = WhisperListenBody {
    ///    duration: 5,
    ///    model: String::from("small"),
    /// };
    /// let user_response = whisper.listen_and_transcribe(listen_body).unwrap();
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// Cleans up any temporary files created during the recording session.
    /// This should be called after the recording session is finished or transcription.
    fn clean(&self) {
        let audio_path = "temp.mp3";
        if Path::new(audio_path).exists() {
            fs::remove_file(&audio_path).unwrap_or_default();
        }
        let json_path = audio_path.replace(".mp3", ".json");
        if Path::new(&json_path).exists() {
            fs::remove_file(&json_path).unwrap_or_default();
        }
        if Path::new("logs").exists() {
            fs::remove_file("logs").unwrap_or_default();
        }
    }

    /// Starts a new recording session with the default input device and the
    /// given duration in seconds. Will return the path to the recording file.
    ///
    /// # Examples
    ///
    /// ```
    /// let whisper = Whisper::new();
    /// let recording = whisper.start_recording(5);
    /// ```
    pub fn listen(&self, duration: i32) -> Result<&Path, WhisperError> {
        // Overwrite the file if it already exists.
        fs::remove_file("temp.mp3").unwrap_or_default();

        // Uses the default input device for the system ":0" and records for the specified duration.
        let recording =
            run_fun!(ffmpeg -f avfoundation -i ":0" -t $duration "temp.mp3"  > logs 2>&1);
        if recording.is_err() {
            return Err(WhisperError::recording_error());
        }

        Ok(Path::new("temp.mp3"))
    }

    /// Transcribes the audio file at the given location.
    ///
    /// Models are available at https://github.com/openai/whisper/blob/main/model-card.md
    ///
    /// # Examples
    ///
    /// ```
    /// let whisper = Whisper::new();
    /// let transcription_response: Result<WhisperTranscriptionResponse, WhisperError> = whisper.transcribe("temp.mp3", "small").unwrap();
    /// ```
    pub fn transcribe(
        &self,
        audio_file_location: &Path,
        model: String,
    ) -> Result<WhisperTranscriptionResponse, WhisperError> {
        println!("Transcribing...");

        // Transcribe the audio file.
        let path = match audio_file_location.to_str() {
            Some(p) => p,
            None => return Err(WhisperError::transcription_error()),
        };
        let _ = match run_fun!(whisper audio $path --model $model --output_format json > logs 2>&1)
        {
            Ok(output) => output,
            Err(_) => return Err(WhisperError::transcription_error()),
        };

        // Read the transcription JSON file.
        let json_file_location = path.replace(".mp3", ".json");
        let transcription_json = match fs::read_to_string(json_file_location) {
            Ok(json) => json,
            Err(_) => return Err(WhisperError::transcription_error()),
        };

        Ok(serde_json::from_str::<WhisperTranscriptionResponse>(&transcription_json).unwrap())
    }

    /// Listens for audio input and transcribes it.
    ///
    /// # Examples
    ///
    /// ```
    /// let whisper = Whisper::new();
    /// let listen_body = WhisperListenBody {
    ///    duration: 5,
    ///    model: String::from("small"),
    /// };
    /// let user_response: Result<WhisperTranscriptionResponse, WhisperError> = whisper.listen_and_transcribe(listen_body).unwrap();
    /// ```
    pub fn listen_and_transcribe(
        &self,
        body: WhisperListenBody,
    ) -> Result<WhisperTranscriptionResponse, WhisperError> {
        let out_location = match self.listen(body.duration) {
            Ok(location) => location,
            Err(e) => return Err(e),
        };

        let res = self.transcribe(&out_location, body.model);

        self.clean();

        res
    }

    /// Returns the default input device for the system.
    ///
    /// # Examples
    ///
    /// ```
    /// let whisper = Whisper::new();
    /// let default_input_device = whisper.get_default_input_device();
    /// ```
    pub fn get_default_input_device(&self) -> Option<String> {
        let available_hosts = cpal::available_hosts();

        for host_id in available_hosts {
            let host = match cpal::host_from_id(host_id) {
                Ok(h) => h,
                Err(_) => continue,
            };

            let default_in = host.default_input_device().map(|e| e.name().unwrap());

            return default_in;
        }

        None
    }
}
