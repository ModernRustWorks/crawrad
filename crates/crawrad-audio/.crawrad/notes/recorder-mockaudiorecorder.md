---
id: recorder-mockaudiorecorder
kind: doc
target_file: src/recorder.rs
line_hint: 38
author: 
created: 
title: "MockAudioRecorder"
tags: []
signature: "pub struct MockAudioRecorder"
---

Mock recorder for unit testing and headless CI.

# Examples

```rust
use crawrad_audio::MockAudioRecorder;

let recorder = MockAudioRecorder::new(vec![0.1, 0.2]);
```
