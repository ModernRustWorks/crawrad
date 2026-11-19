---
id: transcriber-speechtranscriber
kind: doc
target_file: src/transcriber.rs
line_hint: 41
author: 
created: 
title: "SpeechTranscriber"
tags: []
signature: "pub trait SpeechTranscriber : Send + Sync"
---

Abstract contract for Speech-to-Text transcription.

# Examples

```rust
use crawrad_audio::{SpeechTranscriber, TranscriptionConfig};
use crawrad_audio::transcriber::MockTranscriber;

let transcriber = MockTranscriber::new("text", "en");
let result = transcriber.transcribe(&[0.1; 1600], &TranscriptionConfig::default()).unwrap();
assert_eq!(result.text, "text");
```
