//! Error types for audio recording, transcription and speech synthesis.

use crawrad_note::note;
use thiserror::Error;

#[note("error-audioerror")]
#[derive(Debug, Error)]
pub enum AudioError {
    /// Audio device initialization or access error.
    #[error("Audio device error: {0}")]
    DeviceError(String),

    /// Audio stream creation or playback error.
    #[error("Stream error: {0}")]
    StreamError(String),

    /// Error during transcription processing (Whisper or cloud backend).
    #[error("Transcription error: {0}")]
    TranscriptionError(String),

    /// Error during speech synthesis (TTS).
    #[error("Synthesis error: {0}")]
    SynthesisError(String),

    /// No audio input device found on the system.
    #[error("No audio input device found")]
    NoInputDevice,

    /// No audio recorded or empty buffer.
    #[error("No audio recorded (buffer was empty)")]
    EmptyBuffer,

    /// General I/O or serialization error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
