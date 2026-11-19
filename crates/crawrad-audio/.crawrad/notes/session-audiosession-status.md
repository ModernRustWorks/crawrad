---
id: session-audiosession-status
kind: doc
target_file: src/session.rs
line_hint: 140
author: 
created: 
title: "AudioSession::status"
tags: []
signature: "pub fn status (& self) -> & AudioSessionStatus"
---

Returns the current session status.

# Examples

```rust
use crawrad_audio::{AudioSession, AudioSessionStatus, MockAudioRecorder};

let rec = MockAudioRecorder::new(vec![]);
let session = AudioSession::new(rec, || {});
assert_eq!(session.status(), &AudioSessionStatus::Idle);
```
