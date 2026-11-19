//! `crawrad-audio`: Cross-platform, multilingual audio subsystem for CrawRAD.
//!
//! Provides hardware microphone capture (CPAL), speech transcription (Whisper),
//! and speech synthesis (Piper) with clean abstractions for GUI integration.
#![forbid(unsafe_code)]

pub mod error;
pub mod prompt;
pub mod recorder;
pub mod session;
pub mod synthesizer;
pub mod transcriber;

pub use error::AudioError;
pub use recorder::{AudioConfig, AudioRecorder, MockAudioRecorder};
pub use session::{AudioSession, AudioSessionStatus};
pub use synthesizer::{SpeechSynthesisConfig, VoiceSynthesizer};
pub use transcriber::{
    DefaultTranscriber, SpeechTranscriber, TranscriptionConfig, TranscriptionResult, WavEncoder,
};

#[cfg(all(feature = "cpal", not(crawrad_cross_check)))]
pub use recorder::CpalAudioRecorder;

/// Only in the gate's Apple cross-check from Windows (`--cfg crawrad_cross_check`,
/// `gate.ps1`): `cpal` is not compiled there, and the IDE still names this
/// type. Never in a build anyone runs.
#[cfg(crawrad_cross_check)]
pub use recorder::MockAudioRecorder as CpalAudioRecorder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_recorder_captures_samples() {
        let input_samples = vec![0.1, -0.2, 0.3, -0.4];
        let mut rec = MockAudioRecorder::new(input_samples.clone());

        assert!(!rec.is_recording());
        rec.start_recording().unwrap();
        assert!(rec.is_recording());
        assert!(rec.current_level() > 0.0);

        let captured = rec.stop_recording().unwrap();
        assert_eq!(captured, input_samples);
        assert!(!rec.is_recording());
        assert_eq!(rec.current_level(), 0.0);
    }

    #[test]
    fn wav_encoder_creates_valid_header() {
        let samples = vec![0.0f32; 16000]; // 1s of silence
        let wav = WavEncoder::encode_pcm16_mono(&samples, 16000);

        // Header check
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[12..16], b"fmt ");
        assert_eq!(&wav[36..40], b"data");

        // 44 byte header + 16000 * 2 bytes = 32044 bytes
        assert_eq!(wav.len(), 44 + 32000);
    }

    #[test]
    fn mock_transcriber_converts_samples() {
        let transcriber = transcriber::MockTranscriber::new("Hello world", "pt");
        let samples = vec![0.0f32; 8000];
        let cfg = TranscriptionConfig::default();

        let result = transcriber.transcribe(&samples, &cfg).unwrap();
        assert_eq!(result.text, "Hello world");
        assert_eq!(result.language, "pt");
        assert_eq!(result.duration_secs, 0.5);
    }

    #[test]
    fn audio_session_lifecycle() {
        let samples = vec![0.1f32; 16000];
        let rec = MockAudioRecorder::new(samples);
        let mut session = AudioSession::new(rec, || {});

        assert_eq!(session.status(), &AudioSessionStatus::Idle);
        session.start_recording().unwrap();
        assert!(session.is_recording());

        // Stop and transcribe
        let tr = transcriber::MockTranscriber::new("Build a screen", "pt");
        session
            .stop_and_transcribe(tr, TranscriptionConfig::default())
            .unwrap();

        // Spin until worker finishes
        loop {
            match session.poll() {
                AudioSessionStatus::Finished(res) => {
                    assert_eq!(res.text, "Build a screen");
                    break;
                }
                AudioSessionStatus::Transcribing => {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
                status => panic!("Unexpected status: {status:?}"),
            }
        }
    }

    #[test]
    fn audio_session_waker_called_on_status_changes() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static WAKE_COUNT: AtomicUsize = AtomicUsize::new(0);
        fn waker() {
            WAKE_COUNT.fetch_add(1, Ordering::SeqCst);
        }

        WAKE_COUNT.store(0, Ordering::SeqCst);
        let samples = vec![0.1f32; 16000];
        let rec = MockAudioRecorder::new(samples);
        let mut session = AudioSession::new(rec, waker);

        session.start_recording().unwrap();
        assert_eq!(WAKE_COUNT.load(Ordering::SeqCst), 1);

        let tr = transcriber::MockTranscriber::new("Build a screen", "pt");
        session
            .stop_and_transcribe(tr, TranscriptionConfig::default())
            .unwrap();
        assert_eq!(WAKE_COUNT.load(Ordering::SeqCst), 2);

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
        while WAKE_COUNT.load(Ordering::SeqCst) < 3 && std::time::Instant::now() < deadline {
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        assert_eq!(WAKE_COUNT.load(Ordering::SeqCst), 3);
        match session.poll() {
            AudioSessionStatus::Finished(res) => assert_eq!(res.text, "Build a screen"),
            status => panic!("Unexpected status: {status:?}"),
        }
    }

    #[test]
    fn cpal_device_check() {
        #[cfg(all(feature = "cpal", not(crawrad_cross_check)))]
        {
            use cpal::traits::{DeviceTrait, HostTrait};
            let host = cpal::default_host();
            let dev = host.default_input_device();
            println!("Default input device: {:?}", dev.as_ref().map(|d| d.name()));
            if let Some(d) = dev {
                println!("Default input config: {:?}", d.default_input_config());
            }
        }
    }
}
