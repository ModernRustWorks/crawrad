//! Text-to-Speech (TTS) speech synthesis contracts.

use crate::error::AudioError;
use crawrad_note::note;

#[note("synthesizer-speechsynthesisconfig")]
#[derive(Debug, Clone)]
pub struct SpeechSynthesisConfig {
    /// Voice identifier or model name (e.g. "pt_BR-faber-medium", "en_US-lessac-medium").
    pub voice: String,
    /// Speaking rate factor (1.0 = normal).
    pub rate: f32,
    /// Pitch adjustment factor (1.0 = normal).
    pub pitch: f32,
}

impl Default for SpeechSynthesisConfig {
    fn default() -> Self {
        Self {
            voice: "default".into(),
            rate: 1.0,
            pitch: 1.0,
        }
    }
}

#[note("synthesizer-voicesynthesizer")]
pub trait VoiceSynthesizer: Send + Sync {
    #[note("synthesizer-voicesynthesizer-synthesize")]
    fn synthesize(
        &self,
        text: &str,
        config: &SpeechSynthesisConfig,
    ) -> Result<Vec<f32>, AudioError>;
}

#[note("synthesizer-mockvoicesynthesizer")]
#[derive(Default)]
pub struct MockVoiceSynthesizer;

impl VoiceSynthesizer for MockVoiceSynthesizer {
    fn synthesize(
        &self,
        text: &str,
        _config: &SpeechSynthesisConfig,
    ) -> Result<Vec<f32>, AudioError> {
        if text.trim().is_empty() {
            return Err(AudioError::EmptyBuffer);
        }
        // Generate 0.5s of synthetic sine wave
        let sample_rate = 16000;
        let num_samples = sample_rate / 2;
        let mut samples = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let sample = (t * 440.0 * 2.0 * std::f32::consts::PI).sin() * 0.1;
            samples.push(sample);
        }
        Ok(samples)
    }
}
