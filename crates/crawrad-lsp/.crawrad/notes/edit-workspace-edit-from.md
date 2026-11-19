---
id: edit-workspace-edit-from
kind: doc
target_file: src/edit.rs
line_hint: 26
author: 
created: 
title: "workspace_edit_from"
tags: []
signature: "pub fn workspace_edit_from (value : serde_json :: Value) -> Result < WorkspaceEdit , String >"
---

Parses a `WorkspaceEdit` from server-provided JSON.

A `WorkspaceEdit` as the server sent it: `changes` (URI → edits) or
`documentChanges` (text-document edits). A file create/rename/delete in
`documentChanges` is not taken — the client does not announce
`resourceOperations`, so a server that sends one anyway gets `Err` with
its kind, never a half-applied rename.

# Examples

```rust
use crawrad_lsp::edit::workspace_edit_from;
use serde_json::json;

let edit = workspace_edit_from(json!(null)).unwrap();
assert!(edit.is_empty());
```

# Errors

Returns an error if deserialization fails or if unsupported resource operations are requested.
