---
id: synthesizer-voicesynthesizer-synthesize
kind: doc
target_file: src/synthesizer.rs
line_hint: 29
author: 
created: 
title: "VoiceSynthesizer::synthesize"
tags: []
signature: "fn synthesize (& self , text : & str , config : & SpeechSynthesisConfig ,) -> Result < Vec < f32 > , AudioError >"
---

Synthesizes text into 22.05kHz or 16kHz float32 PCM samples.

# Examples

```rust
use crawrad_audio::{SpeechSynthesisConfig, VoiceSynthesizer};
use crawrad_audio::synthesizer::MockVoiceSynthesizer;

let synth = MockVoiceSynthesizer;
let pcm = synth.synthesize("hi", &SpeechSynthesisConfig::default()).unwrap();
assert!(!pcm.is_empty());
```

# Errors

Returns [`AudioError`](crate::AudioError) if synthesis fails or input is empty.
