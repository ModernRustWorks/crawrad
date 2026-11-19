---
id: prompt-transcribed-status
kind: doc
target_file: src/prompt.rs
line_hint: 46
author: 
created: 
title: "transcribed_status"
tags: []
signature: "pub fn transcribed_status (result : & TranscriptionResult) -> String"
---

The status bar after the transcription.

# Examples

```rust
use crawrad_audio::prompt::transcribed_status;
use crawrad_audio::TranscriptionResult;

let res = TranscriptionResult {
    text: "ok".into(),
    language: "en".into(),
    confidence: 1.0,
    duration_secs: 1.0,
};
assert!(transcribed_status(&res).contains("ok"));
```
