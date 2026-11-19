---
id: rustpresenter-for-program
kind: doc
target_file: src/lib.rs
line_hint: 51
author: 
created: 
title: "RustPresenter::for_program"
tags: []
signature: "pub fn for_program (program : & std :: path :: Path) -> Self"
---

Creates a presenter loaded with debug type layouts for a program.

A presenter that also reads `program`'s DWARF or PDB, for the values the tree
cannot give (elements of `Vec<String>`, `Vec<struct>`). A program
without either gives the plain presenter.

# Examples

```rust
use std::path::Path;
use crawrad_dap_rust::RustPresenter;

let presenter = RustPresenter::for_program(Path::new("nonexistent"));
assert!(!presenter.has_layouts());
```
