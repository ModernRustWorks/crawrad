---
id: error-audioerror
kind: doc
target_file: src/error.rs
line_hint: 6
author: 
created: 
title: "AudioError"
tags: []
signature: "pub enum AudioError"
---

Main error type for `crawrad-audio`.

# Examples

```rust
use crawrad_audio::AudioError;

let err = AudioError::NoInputDevice;
assert_eq!(err.to_string(), "No audio input device found");
```
