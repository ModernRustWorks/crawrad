---
id: edit-workspaceedit
kind: doc
target_file: src/edit.rs
line_hint: 23
author: 
created: 
title: "WorkspaceEdit"
tags: []
signature: "pub type WorkspaceEdit = BTreeMap < String , Vec < TextEdit > >"
---

The edits of a rename, by file URI, in a stable order.

# Examples

```rust
use crawrad_lsp::edit::WorkspaceEdit;

let edit = WorkspaceEdit::new();
assert!(edit.is_empty());
```
