//! Audio session controller for GUI integration.
//!
//! Manages non-blocking recording, level monitoring and background transcription.

use crate::{
    error::AudioError,
    recorder::AudioRecorder,
    transcriber::{SpeechTranscriber, TranscriptionConfig, TranscriptionResult},
};
use crawrad_note::note;
use std::sync::mpsc::{Receiver, channel};
use std::time::Instant;

#[note("session-audiosessionstatus")]
#[derive(Debug, Clone, PartialEq)]
pub enum AudioSessionStatus {
    /// Inactive / ready to record.
    Idle,
    /// Currently capturing audio.
    Recording {
        /// Elapsed time since recording started.
        elapsed_secs: f32,
        /// Current RMS volume level (0.0 to 1.0).
        level: f32,
    },
    /// Audio captured; processing transcription in the background.
    Transcribing,
    /// Successfully transcribed text.
    Finished(TranscriptionResult),
    /// An error occurred during capture or transcription.
    Error(String),
}

#[note("session-audiosession")]
pub struct AudioSession<R: AudioRecorder> {
    recorder: R,
    started_at: Option<Instant>,
    rx_transcription: Option<Receiver<Result<TranscriptionResult, AudioError>>>,
    status: AudioSessionStatus,
    waker: fn(),
}

impl<R: AudioRecorder> AudioSession<R> {
    #[note("session-audiosession-new")]
    pub fn new(recorder: R, waker: fn()) -> Self {
        Self {
            recorder,
            started_at: None,
            rx_transcription: None,
            status: AudioSessionStatus::Idle,
            waker,
        }
    }

    #[note("session-audiosession-start-recording")]
    pub fn start_recording(&mut self) -> Result<(), AudioError> {
        self.recorder.start_recording()?;
        self.started_at = Some(Instant::now());
        self.status = AudioSessionStatus::Recording {
            elapsed_secs: 0.0,
            level: 0.0,
        };
        (self.waker)();
        Ok(())
    }

    #[note("session-audiosession-stop-and-transcribe")]
    pub fn stop_and_transcribe<T: SpeechTranscriber + 'static>(
        &mut self,
        transcriber: T,
        config: TranscriptionConfig,
    ) -> Result<(), AudioError> {
        let samples = self.recorder.stop_recording()?;
        self.started_at = None;
        self.status = AudioSessionStatus::Transcribing;
        (self.waker)();

        let (tx, rx) = channel();
        self.rx_transcription = Some(rx);
        let waker = self.waker;

        std::thread::Builder::new()
            .name("crawrad-transcription".into())
            .spawn(move || {
                let res = transcriber.transcribe(&samples, &config);
                let _ = tx.send(res);
                waker();
            })
            .map_err(|e| AudioError::StreamError(format!("Failed to spawn worker: {e}")))?;

        Ok(())
    }

    #[note("session-audiosession-cancel")]
    pub fn cancel(&mut self) {
        if self.recorder.is_recording() {
            let _ = self.recorder.stop_recording();
        }
        self.started_at = None;
        self.rx_transcription = None;
        self.status = AudioSessionStatus::Idle;
        (self.waker)();
    }

    #[note("session-audiosession-poll")]
    pub fn poll(&mut self) -> &AudioSessionStatus {
        if let AudioSessionStatus::Recording { .. } = self.status {
            let elapsed = self
                .started_at
                .map(|t| t.elapsed().as_secs_f32())
                .unwrap_or(0.0);
            let level = self.recorder.current_level();
            self.status = AudioSessionStatus::Recording {
                elapsed_secs: elapsed,
                level,
            };
        } else if let Some(rx) = &self.rx_transcription {
            match rx.try_recv() {
                Ok(Ok(result)) => {
                    self.status = AudioSessionStatus::Finished(result);
                    self.rx_transcription = None;
                }
                Ok(Err(err)) => {
                    self.status = AudioSessionStatus::Error(err.to_string());
                    self.rx_transcription = None;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    // Still processing
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.status =
                        AudioSessionStatus::Error("Transcription thread disconnected".into());
                    self.rx_transcription = None;
                }
            }
        }
        &self.status
    }

    #[note("session-audiosession-status")]
    pub fn status(&self) -> &AudioSessionStatus {
        &self.status
    }

    #[note("session-audiosession-is-recording")]
    pub fn is_recording(&self) -> bool {
        self.recorder.is_recording()
    }

    #[note("session-audiosession-recorder")]
    pub fn recorder(&self) -> &R {
        &self.recorder
    }

    #[note("session-audiosession-recorder-mut")]
    pub fn recorder_mut(&mut self) -> &mut R {
        &mut self.recorder
    }
}
