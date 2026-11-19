---
id: recorder-audiorecorder-is-recording
kind: doc
target_file: src/recorder.rs
line_hint: 31
author: 
created: 
title: "AudioRecorder::is_recording"
tags: []
signature: "fn is_recording (& self) -> bool"
---

Returns whether the recorder is currently capturing audio.

# Examples

```rust
use crawrad_audio::{AudioRecorder, MockAudioRecorder};

let recorder = MockAudioRecorder::new(vec![]);
assert!(!recorder.is_recording());
```
