---
id: lsp-cell-lspcell-running
kind: doc
target_file: src/lsp_cell.rs
line_hint: 48
author: 
created: 
title: "LspCell::running"
tags: []
signature: "pub fn running (name : & str , progress : Option < & str > , idle_note : Option < & str >) -> Self"
---

Creates cell state for a running language server with progress details.

A server running: busy with its progress line, otherwise ready, and
then `idle_note` (e.g. the "not generated" sentence) or nothing.

# Examples

```rust
use crawrad_insight::lsp_cell::{LspCell, LspPhase};

let cell = LspCell::running("rust-analyzer", None, None);
assert_eq!(cell.phase, LspPhase::Ready);
```
