---
id: transcriber-transcriptionresult
kind: doc
target_file: src/transcriber.rs
line_hint: 28
author: 
created: 
title: "TranscriptionResult"
tags: []
signature: "pub struct TranscriptionResult"
---

Result of an audio transcription.

# Examples

```rust
use crawrad_audio::TranscriptionResult;

let result = TranscriptionResult {
    text: "ok".into(),
    language: "en".into(),
    confidence: 1.0,
    duration_secs: 0.5,
};
assert_eq!(result.text, "ok");
```
