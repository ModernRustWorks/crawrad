---
id: recorder-audiorecorder-start-recording
kind: doc
target_file: src/recorder.rs
line_hint: 25
author: 
created: 
title: "AudioRecorder::start_recording"
tags: []
signature: "fn start_recording (& mut self) -> Result < () , AudioError >"
---

Starts capturing audio in the background.

# Examples

```rust
use crawrad_audio::{AudioRecorder, MockAudioRecorder};

let mut recorder = MockAudioRecorder::new(vec![]);
recorder.start_recording().unwrap();
assert!(recorder.is_recording());
```

# Errors

Returns [`AudioError`](crate::AudioError) if the audio device fails to start.
