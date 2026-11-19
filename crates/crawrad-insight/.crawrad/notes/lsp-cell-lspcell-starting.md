---
id: lsp-cell-lspcell-starting
kind: doc
target_file: src/lsp_cell.rs
line_hint: 56
author: 
created: 
title: "LspCell::starting"
tags: []
signature: "pub fn starting (name : & str) -> Self"
---

A server starting: busy, detail `starting…`.

# Examples

```rust
use crawrad_insight::lsp_cell::{LspCell, LspPhase};

let cell = LspCell::starting("rust-analyzer");
assert_eq!(cell.phase, LspPhase::Busy);
```
