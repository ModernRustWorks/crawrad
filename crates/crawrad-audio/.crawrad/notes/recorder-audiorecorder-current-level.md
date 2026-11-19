---
id: recorder-audiorecorder-current-level
kind: doc
target_file: src/recorder.rs
line_hint: 34
author: 
created: 
title: "AudioRecorder::current_level"
tags: []
signature: "fn current_level (& self) -> f32"
---

Current audio RMS volume level (0.0 to 1.0) for UI waveforms/meters.

# Examples

```rust
use crawrad_audio::{AudioRecorder, MockAudioRecorder};

let recorder = MockAudioRecorder::new(vec![]);
assert_eq!(recorder.current_level(), 0.0);
```
