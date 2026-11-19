---
id: session-audiosession-cancel
kind: doc
target_file: src/session.rs
line_hint: 94
author: 
created: 
title: "AudioSession::cancel"
tags: []
signature: "pub fn cancel (& mut self)"
---

Cancels active recording without transcribing.

# Examples

```rust
use crawrad_audio::{AudioSession, AudioSessionStatus, MockAudioRecorder};

let rec = MockAudioRecorder::new(vec![]);
let mut session = AudioSession::new(rec, || {});
session.start_recording().unwrap();
session.cancel();
assert_eq!(session.status(), &AudioSessionStatus::Idle);
```
