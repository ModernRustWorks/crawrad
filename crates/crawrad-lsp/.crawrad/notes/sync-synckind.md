---
id: sync-synckind
kind: doc
target_file: src/sync.rs
line_hint: 18
author: 
created: 
title: "SyncKind"
tags: []
signature: "pub enum SyncKind"
---

What the server accepts in `textDocument/didChange`.

# Examples

```rust
use crawrad_lsp::sync::SyncKind;

let kind = SyncKind::Incremental;
assert_eq!(kind, SyncKind::Incremental);
```
