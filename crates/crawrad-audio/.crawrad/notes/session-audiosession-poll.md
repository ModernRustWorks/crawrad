---
id: session-audiosession-poll
kind: doc
target_file: src/session.rs
line_hint: 105
author: 
created: 
title: "AudioSession::poll"
tags: []
signature: "pub fn poll (& mut self) -> & AudioSessionStatus"
---

Checks for status updates (call on each UI frame or timer tick).

# Examples

```rust
use crawrad_audio::{AudioSession, AudioSessionStatus, MockAudioRecorder};

let rec = MockAudioRecorder::new(vec![]);
let mut session = AudioSession::new(rec, || {});
assert_eq!(session.poll(), &AudioSessionStatus::Idle);
```
