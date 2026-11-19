---
id: session-audiosession-start-recording
kind: doc
target_file: src/session.rs
line_hint: 55
author: 
created: 
title: "AudioSession::start_recording"
tags: []
signature: "pub fn start_recording (& mut self) -> Result < () , AudioError >"
---

Starts audio recording.

# Examples

```rust
use crawrad_audio::{AudioSession, MockAudioRecorder};

let rec = MockAudioRecorder::new(vec![]);
let mut session = AudioSession::new(rec, || {});
session.start_recording().unwrap();
assert!(session.is_recording());
```

# Errors

Returns [`AudioError`](crate::AudioError) if the audio recorder fails to start.
