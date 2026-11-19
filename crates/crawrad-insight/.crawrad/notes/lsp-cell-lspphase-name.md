---
id: lsp-cell-lspphase-name
kind: doc
target_file: src/lsp_cell.rs
line_hint: 29
author: 
created: 
title: "LspPhase::name"
tags: []
signature: "pub fn name (self) -> & 'static str"
---

`"busy"`, `"ready"`, `"failed"` — what the MCP reports.

# Examples

```rust
use crawrad_insight::lsp_cell::LspPhase;

assert_eq!(LspPhase::Ready.name(), "ready");
```
