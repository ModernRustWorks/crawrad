---
id: symbols-symbol
kind: doc
target_file: src/symbols.rs
line_hint: 20
author: 
created: 
title: "Symbol"
tags: []
signature: "pub struct Symbol"
---

One item the server knows: its name, the type it belongs to, and where.

# Examples

```rust
use crawrad_lsp::symbols::Symbol;
use std::path::PathBuf;

let sym = Symbol {
    name: "run".to_string(),
    container: Some("App".to_string()),
    kind: 6,
    file: PathBuf::from("src/main.rs"),
    line: 10,
};
assert_eq!(sym.name, "run");
```
