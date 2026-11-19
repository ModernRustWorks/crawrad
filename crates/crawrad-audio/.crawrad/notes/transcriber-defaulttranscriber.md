---
id: transcriber-defaulttranscriber
kind: doc
target_file: src/transcriber.rs
line_hint: 144
author: 
created: 
title: "DefaultTranscriber"
tags: []
signature: "pub struct DefaultTranscriber"
---

Default transcriber for local execution using Whisper STT.

Uses `faster-whisper` (Python backend) when available on the system
for high-speed local offline transcription (in Portuguese, English, etc.).
Falls back to mock or heuristic transcription if Python / Whisper is not installed.

# Examples

```rust
use crawrad_audio::DefaultTranscriber;

let transcriber = DefaultTranscriber::default();
```
