---
id: prompt-promptlook
kind: doc
target_file: src/prompt.rs
line_hint: 57
author: 
created: 
title: "PromptLook"
tags: []
signature: "pub struct PromptLook"
---

The prompt box and its send button, recording (dimmed, read-only) or not.

# Examples

```rust
use crawrad_audio::prompt::look;

let ready = look(false);
assert_eq!(ready.disabled, 0.0);
```
