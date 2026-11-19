---
id: memory-aefosmemory
kind: doc
target_file: src/memory.rs
line_hint: 5
author: 
created: 
title: "AefosMemory"
tags: []
signature: "pub struct AefosMemory"
---

Global AI memory and persistent user instructions.

Global AI memory and persistent user instructions (Ref 03: `03_modal_memory.png`).

# Examples

```rust
use crawrad_agent::AefosMemory;

let mem = AefosMemory::new("Always write safe code.");
assert_eq!(mem.character_count(), 23);
```
