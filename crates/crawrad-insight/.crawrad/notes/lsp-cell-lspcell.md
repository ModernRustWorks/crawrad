---
id: lsp-cell-lspcell
kind: doc
target_file: src/lsp_cell.rs
line_hint: 39
author: 
created: 
title: "LspCell"
tags: []
signature: "pub struct LspCell"
---

The pill (`name`, its icon's `phase`) and the progress cell beside it.

# Examples

```rust
use crawrad_insight::lsp_cell::{LspCell, LspPhase};

let cell = LspCell::default();
assert_eq!(cell.phase, LspPhase::Ready);
```
