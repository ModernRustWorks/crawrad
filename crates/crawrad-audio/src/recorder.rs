//! Audio recording abstractions and CPAL cross-platform implementation.

use crate::error::AudioError;

#[note("recorder-audioconfig")]
#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// Target sample rate in Hz (default: 16000 for Whisper STT).
    pub sample_rate: u32,
    /// Number of audio channels (default: 1 for mono).
    pub channels: u16,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            channels: 1,
        }
    }
}

#[note("recorder-audiorecorder")]
pub trait AudioRecorder {
    #[note("recorder-audiorecorder-start-recording")]
    fn start_recording(&mut self) -> Result<(), AudioError>;

    #[note("recorder-audiorecorder-stop-recording")]
    fn stop_recording(&mut self) -> Result<Vec<f32>, AudioError>;

    #[note("recorder-audiorecorder-is-recording")]
    fn is_recording(&self) -> bool;

    #[note("recorder-audiorecorder-current-level")]
    fn current_level(&self) -> f32;
}

#[note("recorder-mockaudiorecorder")]
#[derive(Default)]
pub struct MockAudioRecorder {
    recording: bool,
    samples_to_produce: Vec<f32>,
    level: f32,
}

impl MockAudioRecorder {
    #[note("recorder-mockaudiorecorder-new")]
    pub fn new(samples: Vec<f32>) -> Self {
        Self {
            recording: false,
            samples_to_produce: samples,
            level: 0.0,
        }
    }

    #[note("recorder-mockaudiorecorder-set-level")]
    pub fn set_level(&mut self, level: f32) {
        self.level = level.clamp(0.0, 1.0);
    }
}

impl AudioRecorder for MockAudioRecorder {
    fn start_recording(&mut self) -> Result<(), AudioError> {
        self.recording = true;
        self.level = 0.5;
        Ok(())
    }

    fn stop_recording(&mut self) -> Result<Vec<f32>, AudioError> {
        if !self.recording {
            return Err(AudioError::EmptyBuffer);
        }
        self.recording = false;
        self.level = 0.0;
        let mut out = Vec::new();
        std::mem::swap(&mut out, &mut self.samples_to_produce);
        Ok(out)
    }

    fn is_recording(&self) -> bool {
        self.recording
    }

    fn current_level(&self) -> f32 {
        self.level
    }
}

#[cfg(all(feature = "cpal", not(crawrad_cross_check)))]
mod cpal_impl {
    use super::*;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    };

    pub struct CpalAudioRecorder {
        config: AudioConfig,
        recording: Arc<AtomicBool>,
        buffer: Arc<Mutex<Vec<f32>>>,
        recent_level: Arc<Mutex<f32>>,
        active_stream: Option<cpal::Stream>,
    }

    impl CpalAudioRecorder {
        pub fn new(config: AudioConfig) -> Self {
            Self {
                config,
                recording: Arc::new(AtomicBool::new(false)),
                buffer: Arc::new(Mutex::new(Vec::new())),
                recent_level: Arc::new(Mutex::new(0.0)),
                active_stream: None,
            }
        }

        pub fn config(&self) -> &AudioConfig {
            &self.config
        }
    }

    impl Default for CpalAudioRecorder {
        fn default() -> Self {
            Self::new(AudioConfig::default())
        }
    }

    impl AudioRecorder for CpalAudioRecorder {
        fn start_recording(&mut self) -> Result<(), AudioError> {
            if self.recording.load(Ordering::SeqCst) {
                return Ok(());
            }

            let host = cpal::default_host();
            let device = host
                .default_input_device()
                .ok_or(AudioError::NoInputDevice)?;

            let supported_config = device
                .default_input_config()
                .map_err(|e| AudioError::DeviceError(e.to_string()))?;

            let sample_rate = supported_config.sample_rate().0;
            let channels = supported_config.channels() as usize;
            let target_rate = self.config.sample_rate;

            let buffer = Arc::clone(&self.buffer);
            buffer
                .lock()
                .map_err(|_| AudioError::StreamError("Buffer lock failed".into()))?
                .clear();

            let recent_level = Arc::clone(&self.recent_level);
            let recording = Arc::clone(&self.recording);
            recording.store(true, Ordering::SeqCst);

            let err_fn = |err| {
                eprintln!("[crawrad-audio] An error occurred on input stream: {err}");
            };

            let stream = match supported_config.sample_format() {
                cpal::SampleFormat::F32 => {
                    let buffer_inner = Arc::clone(&buffer);
                    let level_inner = Arc::clone(&recent_level);
                    device.build_input_stream(
                        &supported_config.into(),
                        move |data: &[f32], _: &_| {
                            process_samples(
                                data,
                                channels,
                                sample_rate,
                                target_rate,
                                &buffer_inner,
                                &level_inner,
                            );
                        },
                        err_fn,
                        None,
                    )
                }
                cpal::SampleFormat::I16 => {
                    let buffer_inner = Arc::clone(&buffer);
                    let level_inner = Arc::clone(&recent_level);
                    device.build_input_stream(
                        &supported_config.into(),
                        move |data: &[i16], _: &_| {
                            let f32_data: Vec<f32> =
                                data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                            process_samples(
                                &f32_data,
                                channels,
                                sample_rate,
                                target_rate,
                                &buffer_inner,
                                &level_inner,
                            );
                        },
                        err_fn,
                        None,
                    )
                }
                cpal::SampleFormat::U16 => {
                    let buffer_inner = Arc::clone(&buffer);
                    let level_inner = Arc::clone(&recent_level);
                    device.build_input_stream(
                        &supported_config.into(),
                        move |data: &[u16], _: &_| {
                            let f32_data: Vec<f32> = data
                                .iter()
                                .map(|&s| (s as f32 - 32768.0) / 32768.0)
                                .collect();
                            process_samples(
                                &f32_data,
                                channels,
                                sample_rate,
                                target_rate,
                                &buffer_inner,
                                &level_inner,
                            );
                        },
                        err_fn,
                        None,
                    )
                }
                format => {
                    return Err(AudioError::DeviceError(format!(
                        "Unsupported sample format: {format:?}"
                    )));
                }
            }
            .map_err(|e| AudioError::StreamError(e.to_string()))?;

            stream
                .play()
                .map_err(|e| AudioError::StreamError(e.to_string()))?;

            self.active_stream = Some(stream);
            Ok(())
        }

        fn stop_recording(&mut self) -> Result<Vec<f32>, AudioError> {
            self.recording.store(false, Ordering::SeqCst);
            self.active_stream = None;

            let mut buf = self
                .buffer
                .lock()
                .map_err(|_| AudioError::StreamError("Buffer lock failed".into()))?;

            if buf.is_empty() {
                return Err(AudioError::EmptyBuffer);
            }

            let mut out = Vec::new();
            std::mem::swap(&mut out, &mut *buf);
            Ok(out)
        }

        fn is_recording(&self) -> bool {
            self.recording.load(Ordering::SeqCst)
        }

        fn current_level(&self) -> f32 {
            if !self.is_recording() {
                return 0.0;
            }
            self.recent_level.lock().map(|lvl| *lvl).unwrap_or(0.0)
        }
    }

    fn process_samples(
        data: &[f32],
        channels: usize,
        src_rate: u32,
        target_rate: u32,
        buffer: &Arc<Mutex<Vec<f32>>>,
        level_out: &Arc<Mutex<f32>>,
    ) {
        if data.is_empty() {
            return;
        }

        // Convert multi-channel to mono by averaging channels
        let mono: Vec<f32> = if channels > 1 {
            data.chunks(channels)
                .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
                .collect()
        } else {
            data.to_vec()
        };

        // Calculate RMS volume level
        let sum_sq: f32 = mono.iter().map(|&s| s * s).sum();
        let rms = (sum_sq / mono.len() as f32).sqrt().min(1.0);
        if let Ok(mut lvl) = level_out.lock() {
            *lvl = rms * 2.5; // Amplify for UI visibility
        }

        // Resample to 16kHz if needed (linear interpolation)
        let resampled: Vec<f32> = if src_rate != target_rate {
            let ratio = src_rate as f64 / target_rate as f64;
            let target_len = (mono.len() as f64 / ratio).round() as usize;
            let mut out = Vec::with_capacity(target_len);
            for i in 0..target_len {
                let src_pos = i as f64 * ratio;
                let idx = src_pos.floor() as usize;
                let fract = (src_pos - idx as f64) as f32;
                let s1 = mono.get(idx).copied().unwrap_or(0.0);
                let s2 = mono.get(idx + 1).copied().unwrap_or(s1);
                out.push(s1 + (s2 - s1) * fract);
            }
            out
        } else {
            mono
        };

        if let Ok(mut buf) = buffer.lock() {
            buf.extend_from_slice(&resampled);
        }
    }
}

#[cfg(all(feature = "cpal", not(crawrad_cross_check)))]
pub use cpal_impl::CpalAudioRecorder;
use crawrad_note::note;
