---
id: prompt-after-transcription
kind: doc
target_file: src/prompt.rs
line_hint: 37
author: 
created: 
title: "after_transcription"
tags: []
signature: "pub fn after_transcription (previous : & str , said : & str) -> String"
---

The prompt after the transcription: what was typed, then what was said.

# Examples

```rust
use crawrad_audio::prompt::after_transcription;

assert_eq!(after_transcription("prompt", "voice"), "prompt voice");
```
