---
id: transcriber-defaulttranscriber-transcribe-whisper
kind: doc
target_file: src/transcriber.rs
line_hint: 149
author: 
created: 
title: "DefaultTranscriber::transcribe_whisper"
tags: []
signature: "pub fn transcribe_whisper (& self , samples : & [f32] , config : & TranscriptionConfig ,) -> Result < TranscriptionResult , AudioError >"
---

Transcribes audio samples into text using local Whisper STT (faster-whisper).

# Examples

```rust,no_run
// requires external python environment with faster-whisper installed
use crawrad_audio::{DefaultTranscriber, TranscriptionConfig};

let transcriber = DefaultTranscriber::default();
let samples = vec![0.0f32; 16000];
let _ = transcriber.transcribe_whisper(&samples, &TranscriptionConfig::default());
```

# Errors

Returns [`AudioError`](crate::AudioError) if Python execution or Whisper transcription fails.
