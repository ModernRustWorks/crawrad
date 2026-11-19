---
id: memory-aefosmemory-estimated-tokens
kind: doc
target_file: src/memory.rs
line_hint: 28
author: 
created: 
title: "AefosMemory::estimated_tokens"
tags: []
signature: "pub fn estimated_tokens (& self) -> usize"
---

Estimated token count based on character count heuristic.

Estimated token count (roughly 4 characters per token as standard LLM heuristic).

# Examples

```rust
use crawrad_agent::AefosMemory;

let mem = AefosMemory::new("12345678");
assert_eq!(mem.estimated_tokens(), 2);
```
