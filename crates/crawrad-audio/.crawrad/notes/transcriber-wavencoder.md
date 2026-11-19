---
id: transcriber-wavencoder
kind: doc
target_file: src/transcriber.rs
line_hint: 98
author: 
created: 
title: "WavEncoder"
tags: []
signature: "pub struct WavEncoder"
---

In-memory RIFF/WAV encoder for 16-bit PCM audio.

In-memory RIFF/WAV encoder for 16kHz mono 16-bit PCM audio.
Perfect for sending audio to Whisper endpoints or saving to disk.

# Examples

```rust
use crawrad_audio::WavEncoder;

let wav = WavEncoder::encode_pcm16_mono(&[0.0; 160], 16000);
assert_eq!(&wav[0..4], b"RIFF");
```
