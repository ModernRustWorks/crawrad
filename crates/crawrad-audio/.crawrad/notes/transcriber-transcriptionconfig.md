---
id: transcriber-transcriptionconfig
kind: doc
target_file: src/transcriber.rs
line_hint: 6
author: 
created: 
title: "TranscriptionConfig"
tags: []
signature: "pub struct TranscriptionConfig"
---

Configuration options for speech transcription.

# Examples

```rust
use crawrad_audio::TranscriptionConfig;

let config = TranscriptionConfig::default();
assert_eq!(config.temperature, 0.0);
```
