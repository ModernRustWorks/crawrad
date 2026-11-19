---
id: recorder-audiorecorder-stop-recording
kind: doc
target_file: src/recorder.rs
line_hint: 28
author: 
created: 
title: "AudioRecorder::stop_recording"
tags: []
signature: "fn stop_recording (& mut self) -> Result < Vec < f32 > , AudioError >"
---

Stops recording and returns the captured 16kHz mono f32 PCM samples.

# Examples

```rust
use crawrad_audio::{AudioRecorder, MockAudioRecorder};

let mut recorder = MockAudioRecorder::new(vec![0.5]);
recorder.start_recording().unwrap();
let samples = recorder.stop_recording().unwrap();
assert_eq!(samples, vec![0.5]);
```

# Errors

Returns [`AudioError`](crate::AudioError) if recording was not active or buffer capture failed.
