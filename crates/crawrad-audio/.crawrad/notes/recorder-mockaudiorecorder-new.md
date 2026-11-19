---
id: recorder-mockaudiorecorder-new
kind: doc
target_file: src/recorder.rs
line_hint: 47
author: 
created: 
title: "MockAudioRecorder::new"
tags: []
signature: "pub fn new (samples : Vec < f32 >) -> Self"
---

Creates a mock recorder that will produce the given samples upon stop.

# Examples

```rust
use crawrad_audio::{AudioRecorder, MockAudioRecorder};

let mut rec = MockAudioRecorder::new(vec![0.1, 0.2]);
rec.start_recording().unwrap();
assert_eq!(rec.stop_recording().unwrap(), vec![0.1, 0.2]);
```
