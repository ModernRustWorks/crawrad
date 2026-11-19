---
id: memory-aefosmemory-save-to-file
kind: doc
target_file: src/memory.rs
line_hint: 87
author: 
created: 
title: "AefosMemory::save_to_file"
tags: []
signature: "pub fn save_to_file (& self , path : & Path) -> Result < () , std :: io :: Error >"
---

Atomically save memory to disk.

# Errors

Returns an error if directory creation fails or writing to disk fails.

# Examples

```rust,no_run
// Writes memory file to disk.
use crawrad_agent::AefosMemory;
use std::path::Path;

let mem = AefosMemory::new("instructions");
assert!(mem.save_to_file(Path::new("/tmp/memory.json")).is_ok());
```
