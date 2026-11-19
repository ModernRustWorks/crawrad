---
id: session-audiosession
kind: doc
target_file: src/session.rs
line_hint: 34
author: 
created: 
title: "AudioSession"
tags: []
signature: "pub struct AudioSession < R : AudioRecorder >"
---

GUI-friendly audio session coordinator.

# Examples

```rust
use crawrad_audio::{AudioSession, MockAudioRecorder};

let rec = MockAudioRecorder::new(vec![]);
let session = AudioSession::new(rec, || {});
assert!(!session.is_recording());
```
