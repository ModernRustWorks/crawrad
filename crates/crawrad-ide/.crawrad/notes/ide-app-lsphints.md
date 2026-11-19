---
id: ide-app-lsphints
kind: doc
target_file: src/ide_app.rs
line_hint: 371
author: 
created: 
title: "LspHints"
tags: []
signature: "pub type LspHints = crawrad_lsp :: EditorAssist"
---

Editor assist hints and occurrences.

Editor undo history per file (each AI accept becomes 1 undo transaction).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let hints = LspHints::default();
assert!(hints.is_empty());
```
