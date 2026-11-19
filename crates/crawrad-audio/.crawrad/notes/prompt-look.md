---
id: prompt-look
kind: doc
target_file: src/prompt.rs
line_hint: 70
author: 
created: 
title: "look"
tags: []
signature: "pub fn look (recording : bool) -> PromptLook"
---

How the prompt box looks while recording, or not.

# Examples

```rust
use crawrad_audio::prompt::look;

let rec = look(true);
assert_eq!(rec.disabled, 1.0);
```
