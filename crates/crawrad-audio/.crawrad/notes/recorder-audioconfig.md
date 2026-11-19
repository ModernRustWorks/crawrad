---
id: recorder-audioconfig
kind: doc
target_file: src/recorder.rs
line_hint: 5
author: 
created: 
title: "AudioConfig"
tags: []
signature: "pub struct AudioConfig"
---

Configuration for audio capture.

# Examples

```rust
use crawrad_audio::AudioConfig;

let config = AudioConfig::default();
assert_eq!(config.sample_rate, 16000);
assert_eq!(config.channels, 1);
```
