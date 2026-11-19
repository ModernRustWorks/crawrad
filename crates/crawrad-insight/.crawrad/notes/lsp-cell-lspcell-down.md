---
id: lsp-cell-lspcell-down
kind: doc
target_file: src/lsp_cell.rs
line_hint: 61
author: 
created: 
title: "LspCell::down"
tags: []
signature: "pub fn down (name : & str , why : & str) -> Self"
---

No server: failed, and why (`offline`, `stopped`, `exited`, the spawn error).

# Examples

```rust
use crawrad_insight::lsp_cell::{LspCell, LspPhase};

let cell = LspCell::down("rust-analyzer", "offline");
assert_eq!(cell.phase, LspPhase::Failed);
```
