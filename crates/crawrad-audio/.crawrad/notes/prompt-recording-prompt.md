---
id: prompt-recording-prompt
kind: doc
target_file: src/prompt.rs
line_hint: 23
author: 
created: 
title: "recording_prompt"
tags: []
signature: "pub fn recording_prompt (previous : & str , secs : f32) -> String"
---

The prompt while recording, `secs` in.

# Examples

```rust
use crawrad_audio::prompt::recording_prompt;

assert!(recording_prompt("note", 1.0).contains("🔴 Recording"));
```
