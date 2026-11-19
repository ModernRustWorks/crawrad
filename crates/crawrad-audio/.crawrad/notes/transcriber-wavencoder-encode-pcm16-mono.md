---
id: transcriber-wavencoder-encode-pcm16-mono
kind: doc
target_file: src/transcriber.rs
line_hint: 102
author: 
created: 
title: "WavEncoder::encode_pcm16_mono"
tags: []
signature: "pub fn encode_pcm16_mono (samples : & [f32] , sample_rate : u32) -> Vec < u8 >"
---

Encodes 16kHz mono float32 samples into a standard 16-bit PCM WAV byte buffer.

# Examples

```rust
use crawrad_audio::WavEncoder;

let wav = WavEncoder::encode_pcm16_mono(&[0.0; 160], 16000);
assert_eq!(&wav[0..4], b"RIFF");
```
