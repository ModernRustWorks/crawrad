---
id: document-separador
kind: doc
target_file: src/document.rs
line_hint: 194
author: 
created: 
title: "SEPARADOR"
tags: []
signature: "pub const SEPARADOR : & str = \"  ·  \""
---

Separator between the name and the class in the tree line.

What separates the name from the class in a tree line — the same as
`tree.rs:60` of the egui shell.

# Examples

```rust
use crawrad_host::document::SEPARADOR;

assert_eq!(SEPARADOR, "  ·  ");
```
