---
id: session-audiosession-new
kind: doc
target_file: src/session.rs
line_hint: 44
author: 
created: 
title: "AudioSession::new"
tags: []
signature: "pub fn new (recorder : R , waker : fn ()) -> Self"
---

Creates a new audio session with the given recorder and UI waker callback.

# Examples

```rust
use crawrad_audio::{AudioSession, MockAudioRecorder};

let rec = MockAudioRecorder::new(vec![]);
let session = AudioSession::new(rec, || {});
assert!(!session.is_recording());
```
