---
id: transcriber-speechtranscriber-transcribe
kind: doc
target_file: src/transcriber.rs
line_hint: 43
author: 
created: 
title: "SpeechTranscriber::transcribe"
tags: []
signature: "fn transcribe (& self , samples : & [f32] , config : & TranscriptionConfig ,) -> Result < TranscriptionResult , AudioError >"
---

Transcribes 16kHz mono float32 audio samples into text.

# Examples

```rust
use crawrad_audio::{SpeechTranscriber, TranscriptionConfig};
use crawrad_audio::transcriber::MockTranscriber;

let transcriber = MockTranscriber::new("test", "en");
let result = transcriber.transcribe(&[0.0; 1600], &TranscriptionConfig::default()).unwrap();
assert_eq!(result.text, "test");
```

# Errors

Returns [`AudioError`](crate::AudioError) if samples are empty or transcription fails.
