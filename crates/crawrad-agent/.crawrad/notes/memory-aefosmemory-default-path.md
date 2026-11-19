---
id: memory-aefosmemory-default-path
kind: doc
target_file: src/memory.rs
line_hint: 33
author: 
created: 
title: "AefosMemory::default_path"
tags: []
signature: "pub fn default_path () -> PathBuf"
---

Default file location for persistent global memory.

# Examples

```rust
use crawrad_agent::AefosMemory;

let path = AefosMemory::default_path();
assert!(path.ends_with("memory.json"));
```
