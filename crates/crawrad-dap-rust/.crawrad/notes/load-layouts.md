---
id: load-layouts
kind: doc
target_file: src/lib.rs
line_hint: 34
author: 
created: 
title: "load_layouts"
tags: []
signature: "pub fn load_layouts (program : & std :: path :: Path) -> Option < dwarf :: TypeIndex >"
---

Loads type layouts from DWARF or PDB debug information for a program.

The type layouts of `program`: its DWARF (`-gnu`) or, failing that, the
PDB beside it (`-msvc`). `None` when neither can be read.

# Examples

```rust
use std::path::Path;
use crawrad_dap_rust::load_layouts;

assert!(load_layouts(Path::new("nonexistent-binary")).is_none());
```
