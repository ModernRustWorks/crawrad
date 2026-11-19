---
id: memory-aefosmemory-character-count
kind: doc
target_file: src/memory.rs
line_hint: 23
author: 
created: 
title: "AefosMemory::character_count"
tags: []
signature: "pub fn character_count (& self) -> usize"
---

Number of characters in the instructions.

# Examples

```rust
use crawrad_agent::AefosMemory;

let mem = AefosMemory::new("hello");
assert_eq!(mem.character_count(), 5);
```
