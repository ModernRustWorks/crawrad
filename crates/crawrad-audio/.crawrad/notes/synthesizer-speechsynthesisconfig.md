---
id: synthesizer-speechsynthesisconfig
kind: doc
target_file: src/synthesizer.rs
line_hint: 6
author: 
created: 
title: "SpeechSynthesisConfig"
tags: []
signature: "pub struct SpeechSynthesisConfig"
---

Configuration options for speech synthesis.

# Examples

```rust
use crawrad_audio::SpeechSynthesisConfig;

let config = SpeechSynthesisConfig::default();
assert_eq!(config.voice, "default");
```
