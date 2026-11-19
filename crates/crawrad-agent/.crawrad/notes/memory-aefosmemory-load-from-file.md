---
id: memory-aefosmemory-load-from-file
kind: doc
target_file: src/memory.rs
line_hint: 62
author: 
created: 
title: "AefosMemory::load_from_file"
tags: []
signature: "pub fn load_from_file (path : & Path) -> Result < Self , std :: io :: Error >"
---

Loads memory from a JSON file, returning default if missing.

Load memory from a JSON file, or return a default empty instance if file does not exist.

# Errors

Returns an error if reading the file fails or the file contains invalid JSON data.

# Examples

```rust
use crawrad_agent::AefosMemory;
use std::path::Path;

let mem = AefosMemory::load_from_file(Path::new("non_existent_memory.json")).unwrap();
assert!(mem.instructions.is_empty());
```
