---
id: prompt-transcribing
kind: doc
target_file: src/prompt.rs
line_hint: 8
author: 
created: 
title: "TRANSCRIBING"
tags: []
signature: "pub const TRANSCRIBING : & str = \"⏳ Finishing the transcription with Whisper...\""
---

The status bar while the transcription runs.

# Examples

```rust
use crawrad_audio::prompt::TRANSCRIBING;

assert!(TRANSCRIBING.contains("Whisper"));
```
