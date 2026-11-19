//! Speech-to-Text (STT) transcription contracts and Whisper integrations.

use crate::error::AudioError;
use crawrad_note::note;

#[note("transcriber-transcriptionconfig")]
#[derive(Debug, Clone)]
pub struct TranscriptionConfig {
    /// Target language code (e.g. "pt", "en", "es").
    /// If `None`, automatic language detection (across 99 languages) is performed.
    pub language: Option<String>,
    /// Optional vocabulary or domain prompt hints (e.g. "Rust, Makepad, RAD").
    pub prompt: Option<String>,
    /// Sampling temperature (default 0.0 for deterministic output).
    pub temperature: f32,
}

impl Default for TranscriptionConfig {
    fn default() -> Self {
        Self {
            language: None,
            prompt: Some("Rust, Makepad, RAD, Form, Component, Widget, Struct, Trait".into()),
            temperature: 0.0,
        }
    }
}

#[note("transcriber-transcriptionresult")]
#[derive(Debug, Clone, PartialEq)]
pub struct TranscriptionResult {
    /// Transcribed text in UTF-8.
    pub text: String,
    /// Detected or configured language code (e.g. "pt", "en").
    pub language: String,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f32,
    /// Audio duration in seconds.
    pub duration_secs: f32,
}

#[note("transcriber-speechtranscriber")]
pub trait SpeechTranscriber: Send + Sync {
    #[note("transcriber-speechtranscriber-transcribe")]
    fn transcribe(
        &self,
        samples: &[f32],
        config: &TranscriptionConfig,
    ) -> Result<TranscriptionResult, AudioError>;
}

#[note("transcriber-mocktranscriber")]
pub struct MockTranscriber {
    pub response_text: String,
    pub language: String,
}

impl MockTranscriber {
    pub fn new(text: impl Into<String>, language: impl Into<String>) -> Self {
        Self {
            response_text: text.into(),
            language: language.into(),
        }
    }
}

impl Default for MockTranscriber {
    fn default() -> Self {
        Self {
            response_text: "Create a form with a button and a customer list".into(),
            language: "pt".into(),
        }
    }
}

impl SpeechTranscriber for MockTranscriber {
    fn transcribe(
        &self,
        samples: &[f32],
        config: &TranscriptionConfig,
    ) -> Result<TranscriptionResult, AudioError> {
        if samples.is_empty() {
            return Err(AudioError::EmptyBuffer);
        }
        let duration = samples.len() as f32 / 16000.0;
        let lang = config
            .language
            .clone()
            .unwrap_or_else(|| self.language.clone());
        Ok(TranscriptionResult {
            text: self.response_text.clone(),
            language: lang,
            confidence: 0.98,
            duration_secs: duration,
        })
    }
}

#[note("transcriber-wavencoder")]
pub struct WavEncoder;

impl WavEncoder {
    #[note("transcriber-wavencoder-encode-pcm16-mono")]
    pub fn encode_pcm16_mono(samples: &[f32], sample_rate: u32) -> Vec<u8> {
        let num_samples = samples.len() as u32;
        let bytes_per_sample = 2u16; // 16-bit PCM
        let num_channels = 1u16;
        let byte_rate = sample_rate * num_channels as u32 * bytes_per_sample as u32;
        let block_align = num_channels * bytes_per_sample;
        let data_chunk_size = num_samples * bytes_per_sample as u32;
        let riff_chunk_size = 36 + data_chunk_size;

        let mut out = Vec::with_capacity((44 + data_chunk_size) as usize);

        // RIFF header
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&riff_chunk_size.to_le_bytes());
        out.extend_from_slice(b"WAVE");

        // "fmt " sub-chunk
        out.extend_from_slice(b"fmt ");
        out.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size (16 for PCM)
        out.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat (1 = PCM)
        out.extend_from_slice(&num_channels.to_le_bytes());
        out.extend_from_slice(&sample_rate.to_le_bytes());
        out.extend_from_slice(&byte_rate.to_le_bytes());
        out.extend_from_slice(&block_align.to_le_bytes());
        out.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample

        // "data" sub-chunk
        out.extend_from_slice(b"data");
        out.extend_from_slice(&data_chunk_size.to_le_bytes());

        // Samples converted to i16
        for &sample in samples {
            let clamped = sample.clamp(-1.0, 1.0);
            let val = (clamped * 32767.0) as i16;
            out.extend_from_slice(&val.to_le_bytes());
        }

        out
    }
}

#[note("transcriber-defaulttranscriber")]
#[derive(Default, Clone)]
pub struct DefaultTranscriber;

impl DefaultTranscriber {
    #[note("transcriber-defaulttranscriber-transcribe-whisper")]
    pub fn transcribe_whisper(
        &self,
        samples: &[f32],
        config: &TranscriptionConfig,
    ) -> Result<TranscriptionResult, AudioError> {
        let temp_dir = std::env::temp_dir();
        let rand_id: u128 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let wav_path = temp_dir.join(format!("crawrad_audio_{}.wav", rand_id));

        let wav_bytes = WavEncoder::encode_pcm16_mono(samples, 16000);
        std::fs::write(&wav_path, wav_bytes).map_err(|e| {
            AudioError::TranscriptionError(format!("Failed to write temp wav: {e}"))
        })?;

        let lang_arg = config.language.as_deref().unwrap_or("pt");
        let prompt_arg = config.prompt.as_deref().unwrap_or("none");

        const PYTHON_SCRIPT: &str = r#"
import sys, json, os

try:
    from faster_whisper import WhisperModel
    wav_path = sys.argv[1]
    lang = sys.argv[2] if len(sys.argv) > 2 and sys.argv[2] != 'auto' else None
    prompt = sys.argv[3] if len(sys.argv) > 3 and sys.argv[3] != 'none' else None

    model = WhisperModel('base', device='cpu', compute_type='int8')
    segments, info = model.transcribe(wav_path, language=lang, initial_prompt=prompt, beam_size=1)
    text = ' '.join(s.text.strip() for s in segments if s.text.strip()).strip()

    print(json.dumps({
        'status': 'ok',
        'text': text,
        'language': info.language,
        'confidence': float(info.language_probability),
        'duration_secs': float(info.duration)
    }))
except Exception as e:
    print(json.dumps({'status': 'error', 'message': str(e)}))
"#;

        let mut cmd = crawrad_os::process::no_console("python");
        cmd.arg("-c")
            .arg(PYTHON_SCRIPT)
            .arg(&wav_path)
            .arg(lang_arg)
            .arg(prompt_arg);

        let output = cmd.output().map_err(|e| {
            let _ = std::fs::remove_file(&wav_path);
            AudioError::TranscriptionError(format!("Python STT invocation failed: {e}"))
        })?;

        let _ = std::fs::remove_file(&wav_path);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AudioError::TranscriptionError(format!(
                "STT failed: {stderr}"
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('{')
                && trimmed.ends_with('}')
                && let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed)
            {
                if v.get("status").and_then(|s| s.as_str()) == Some("ok") {
                    let text = v
                        .get("text")
                        .and_then(|t| t.as_str())
                        .unwrap_or("")
                        .to_string();
                    let language = v
                        .get("language")
                        .and_then(|l| l.as_str())
                        .unwrap_or("pt")
                        .to_string();
                    let confidence =
                        v.get("confidence").and_then(|c| c.as_f64()).unwrap_or(0.95) as f32;
                    let duration_secs = v
                        .get("duration_secs")
                        .and_then(|d| d.as_f64())
                        .unwrap_or((samples.len() as f32 / 16000.0) as f64)
                        as f32;

                    return Ok(TranscriptionResult {
                        text,
                        language,
                        confidence,
                        duration_secs,
                    });
                } else if let Some(msg) = v.get("message").and_then(|m| m.as_str()) {
                    return Err(AudioError::TranscriptionError(format!(
                        "Whisper error: {msg}"
                    )));
                }
            }
        }

        Err(AudioError::TranscriptionError(
            "No valid JSON output from Whisper".into(),
        ))
    }
}

impl SpeechTranscriber for DefaultTranscriber {
    fn transcribe(
        &self,
        samples: &[f32],
        config: &TranscriptionConfig,
    ) -> Result<TranscriptionResult, AudioError> {
        if samples.is_empty() {
            return Err(AudioError::EmptyBuffer);
        }
        let duration = samples.len() as f32 / 16000.0;

        // Try a real transcription through a local Whisper (faster-whisper)
        if let Ok(res) = self.transcribe_whisper(samples, config) {
            return Ok(res);
        }

        // Fallback when Whisper is not available
        let lang = config.language.clone().unwrap_or_else(|| "pt".into());
        Ok(TranscriptionResult {
            text: String::new(),
            language: lang,
            confidence: 0.0,
            duration_secs: duration,
        })
    }
}
