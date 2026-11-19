---
id: recorder-audiorecorder
kind: doc
target_file: src/recorder.rs
line_hint: 23
author: 
created: 
title: "AudioRecorder"
tags: []
signature: "pub trait AudioRecorder"
---

Abstract contract for recording PCM audio from a microphone.

# Examples

```rust
use crawrad_audio::{AudioRecorder, MockAudioRecorder};

let mut recorder = MockAudioRecorder::new(vec![0.0]);
assert!(!recorder.is_recording());
```
