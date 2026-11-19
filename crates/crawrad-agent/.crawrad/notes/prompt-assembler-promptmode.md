---
id: prompt-assembler-promptmode
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 8
author: 
created: 
title: "PromptMode"
tags: []
signature: "pub enum PromptMode"
---

Operating mode for the prompt assembler.

# Examples

```rust
use crawrad_agent::PromptMode;

let mode = PromptMode::Chat;
assert_eq!(mode, PromptMode::Chat);
```
