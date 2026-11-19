---
id: session-audiosession-recorder-mut
kind: doc
target_file: src/session.rs
line_hint: 155
author: 
created: 
title: "AudioSession::recorder_mut"
tags: []
signature: "pub fn recorder_mut (& mut self) -> & mut R"
---

Mutable access to the underlying recorder.

# Examples

```rust
use crawrad_audio::{AudioRecorder, AudioSession, MockAudioRecorder};

let rec = MockAudioRecorder::new(vec![]);
let mut session = AudioSession::new(rec, || {});
session.recorder_mut().set_level(0.5);
assert_eq!(session.recorder().current_level(), 0.5);
```
