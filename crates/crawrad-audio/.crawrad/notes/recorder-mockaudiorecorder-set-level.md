---
id: recorder-mockaudiorecorder-set-level
kind: doc
target_file: src/recorder.rs
line_hint: 56
author: 
created: 
title: "MockAudioRecorder::set_level"
tags: []
signature: "pub fn set_level (& mut self , level : f32)"
---

Sets the volume level reported by this mock recorder.

# Examples

```rust
use crawrad_audio::{AudioRecorder, MockAudioRecorder};

let mut rec = MockAudioRecorder::new(vec![]);
rec.set_level(0.75);
assert_eq!(rec.current_level(), 0.75);
```
