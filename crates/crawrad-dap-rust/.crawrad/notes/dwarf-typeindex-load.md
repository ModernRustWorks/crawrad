---
id: dwarf-typeindex-load
kind: doc
target_file: src/dwarf.rs
line_hint: 80
author: 
created: 
title: "TypeIndex::load"
tags: []
signature: "pub fn load (path : & Path) -> Option < Self >"
---

Reads DWARF type information from the executable at path.

Reads the DWARF of the executable at `path`. `None` when the file has
no DWARF (a `-msvc` build keeps its types in a PDB instead).

# Examples

```rust
use std::path::Path;
use crawrad_dap_rust::dwarf::TypeIndex;

assert!(TypeIndex::load(Path::new("nonexistent-binary")).is_none());
```
