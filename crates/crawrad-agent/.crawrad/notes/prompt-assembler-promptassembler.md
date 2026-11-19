---
id: prompt-assembler-promptassembler
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 336
author: 
created: 
title: "PromptAssembler"
tags: []
signature: "pub struct PromptAssembler"
---

Assembles the unified structured prompt for the AI assistant or agent.

# Examples

```rust
use crawrad_agent::{AssembleParams, PromptAssembler};

let prompt = PromptAssembler::assemble(AssembleParams::default());
assert!(!prompt.is_empty());
```
