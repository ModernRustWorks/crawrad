---
id: prompt-mic-on
kind: doc
target_file: src/prompt.rs
line_hint: 11
author: 
created: 
title: "MIC_ON"
tags: []
signature: "pub const MIC_ON : & str = \"🔴 Microphone on. Speak now... (click the microphone to finish and enable Send)\""
---

The status bar when the microphone starts.

# Examples

```rust
use crawrad_audio::prompt::MIC_ON;

assert!(MIC_ON.contains("Microphone"));
```
