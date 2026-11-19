---
id: prompt-recording-started
kind: doc
target_file: src/prompt.rs
line_hint: 15
author: 
created: 
title: "recording_started"
tags: []
signature: "pub fn recording_started (previous : & str) -> String"
---

The prompt as recording starts: what was typed, then the listening note.

# Examples

```rust
use crawrad_audio::prompt::recording_started;

assert_eq!(recording_started(""), "Recording audio... (speak now)");
```
