---
id: pdb-load
kind: doc
target_file: src/pdb.rs
line_hint: 26
author: 
created: 
title: "load"
tags: []
signature: "pub fn load (path : & Path) -> Option < TypeIndex >"
---

The layouts in the PDB at `path`, or `None` when it cannot be read.

# Examples

```rust
use std::path::Path;
use crawrad_dap_rust::pdb;

assert!(pdb::load(Path::new("nonexistent.pdb")).is_none());
```
