---
id: session-audiosession-is-recording
kind: doc
target_file: src/session.rs
line_hint: 145
author: 
created: 
title: "AudioSession::is_recording"
tags: []
signature: "pub fn is_recording (& self) -> bool"
---

Returns true if currently recording.

# Examples

```rust
use crawrad_audio::{AudioSession, MockAudioRecorder};

let rec = MockAudioRecorder::new(vec![]);
let session = AudioSession::new(rec, || {});
assert!(!session.is_recording());
```
