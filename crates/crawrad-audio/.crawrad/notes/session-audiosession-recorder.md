---
id: session-audiosession-recorder
kind: doc
target_file: src/session.rs
line_hint: 150
author: 
created: 
title: "AudioSession::recorder"
tags: []
signature: "pub fn recorder (& self) -> & R"
---

Access the underlying recorder.

# Examples

```rust
use crawrad_audio::{AudioRecorder, AudioSession, MockAudioRecorder};

let rec = MockAudioRecorder::new(vec![]);
let session = AudioSession::new(rec, || {});
assert_eq!(session.recorder().current_level(), 0.0);
```
