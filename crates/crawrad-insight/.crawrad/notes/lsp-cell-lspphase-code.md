---
id: lsp-cell-lspphase-code
kind: doc
target_file: src/lsp_cell.rs
line_hint: 20
author: 
created: 
title: "LspPhase::code"
tags: []
signature: "pub fn code (self) -> f64"
---

The icon shader's `phase` instance: 0 busy, 1 ready, 2 failed.

# Examples

```rust
use crawrad_insight::lsp_cell::LspPhase;

assert_eq!(LspPhase::Ready.code(), 1.0);
```
