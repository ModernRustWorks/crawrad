---
id: memory-aefosmemory-save
kind: doc
target_file: src/memory.rs
line_hint: 73
author: 
created: 
title: "AefosMemory::save"
tags: []
signature: "pub fn save (& self) -> Result < () , std :: io :: Error >"
---

Saves memory to the default dedicated CrawRAD location.

# Errors

Returns an error if writing to disk fails.

# Examples

```rust,no_run
// Writes global memory file to disk.
use crawrad_agent::AefosMemory;

let mem = AefosMemory::new("instructions");
assert!(mem.save().is_ok());
```
