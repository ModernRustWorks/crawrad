---
id: prompt-assembler-promptassembler-assemble
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 340
author: 
created: 
title: "PromptAssembler::assemble"
tags: []
signature: "pub fn assemble (params : AssembleParams) -> String"
---

Assembles all prompt sections in a strict, deterministic order.

Assembles all prompt sections in a strict, deterministic order with clear block headers:
1. User Request (priority at the very top so LLMs focus directly on developer goal)
2. Mode Guidelines (Context & Guidelines or Autonomous Agent Guidelines)
3. Global Permanent Instructions (CrawRAD Memory)
4. Project Context (Active Editor File or Designer Form)
5. Referenced Files (@mentions)

Architectural rules (#184/#187):
- `### User Request` and Guidelines preamble are immutable; they are NEVER truncated.
- Total prompt byte budget is strictly respected in bytes (`.len()`), never UTF-8 chars.
- Context budget is dynamically split among the actual number of context blocks.
- Final safeguarding cuts only context content on valid UTF-8 char boundaries.

# Examples

```rust
use crawrad_agent::{AssembleParams, PromptAssembler};

let params = AssembleParams::default();
let prompt = PromptAssembler::assemble(params);
assert!(prompt.contains("User Request"));
```
