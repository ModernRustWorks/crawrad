---
id: prompt-recording-status
kind: doc
target_file: src/prompt.rs
line_hint: 32
author: 
created: 
title: "recording_status"
tags: []
signature: "pub fn recording_status (secs : f32) -> String"
---

The status bar while recording, `secs` in.

# Examples

```rust
use crawrad_audio::prompt::recording_status;

assert!(recording_status(1.0).contains("1.0s"));
```
