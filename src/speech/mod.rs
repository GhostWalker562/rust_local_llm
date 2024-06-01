pub mod errors;

use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use cmd_lib::run_fun;

use errors::SpeechError;
use rodio::{Decoder, OutputStream, Sink};

#[derive(Debug, Clone)]
pub struct Speech {}

impl Speech {
    /// Speech to text client using the coqui-ai TTS.
    ///
    /// # Examples
    /// ```
    /// let speech = Speech::new();
    /// let text = "Hello, how are you?";
    /// let volume = 0.5;
    /// speech.speak(text, volume).unwrap();
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// Cleans up any temporary files created during the speech generation session.
    /// This should be called after the speech generation session is finished.
    fn clean(&self) {
        if Path::new("speech.wav").exists() {
            fs::remove_file("speech.wav").unwrap_or_default();
        }
    }

    /// Play a given audio file with a specified volume to the default output device.
    ///
    /// # Examples
    ///
    /// ```
    /// let speech = Speech::new();
    /// let volume = 0.5;
    /// speech.play("speech.wav", volume).unwrap();
    /// ```
    fn play(&self, input: &str, volume: f32) -> Result<(), SpeechError> {
        let (_stream, handle) =
            OutputStream::try_default().map_err(|_| SpeechError::playback_error())?;

        let sink = Sink::try_new(&handle).map_err(|_| SpeechError::playback_error())?;
        sink.set_volume(volume);

        let file = File::open(input).map_err(|_| SpeechError::playback_error())?;
        let decoder =
            Decoder::new(BufReader::new(file)).map_err(|_| SpeechError::playback_error())?;

        sink.append(decoder);
        sink.sleep_until_end();

        Ok(())
    }

    /// Generates speech wav file from text and plays it.
    ///
    /// # Examples
    ///
    /// ```
    /// let speech = Speech::new();
    /// let text = "Hello, how are you?";
    /// let volume = 0.5;
    /// speech.speak(text, volume).unwrap();
    /// ```
    pub fn speak(&self, text: &str, volume: f32) -> Result<(), SpeechError> {
        // Generate the speech and save it to a file.
        let _ = run_fun!(tts --text $text --out_path speech.wav)
            .map_err(|_| SpeechError::tts_error())?;

        self.play("speech.wav", volume)?;

        self.clean();

        Ok(())
    }
}
