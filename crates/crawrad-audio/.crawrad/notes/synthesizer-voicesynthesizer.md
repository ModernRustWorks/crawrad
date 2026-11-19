---
id: synthesizer-voicesynthesizer
kind: doc
target_file: src/synthesizer.rs
line_hint: 27
author: 
created: 
title: "VoiceSynthesizer"
tags: []
signature: "pub trait VoiceSynthesizer : Send + Sync"
---

Abstract contract for Text-to-Speech synthesis.

# Examples

```rust
use crawrad_audio::{SpeechSynthesisConfig, VoiceSynthesizer};
use crawrad_audio::synthesizer::MockVoiceSynthesizer;

let synth = MockVoiceSynthesizer;
let pcm = synth.synthesize("hello", &SpeechSynthesisConfig::default()).unwrap();
assert!(!pcm.is_empty());
```
