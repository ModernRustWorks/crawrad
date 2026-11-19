---
id: rustpresenter
kind: doc
target_file: src/lib.rs
line_hint: 39
author: 
created: 
title: "RustPresenter"
tags: []
signature: "pub struct RustPresenter"
---

Formats Rust values from DAP variables and debug type layouts.

Formats Rust values from `lldb-dap`'s raw tree and, when the program's
DWARF or PDB was read, from its type layouts.

# Examples

```rust
use crawrad_dap_rust::RustPresenter;

let presenter = RustPresenter::new();
assert!(!presenter.has_layouts());
```
