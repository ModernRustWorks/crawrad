---
id: sync-sync-kind-from
kind: doc
target_file: src/sync.rs
line_hint: 29
author: 
created: 
title: "sync_kind_from"
tags: []
signature: "pub fn sync_kind_from (capabilities : & Value) -> SyncKind"
---

Determines the document sync kind supported by server capabilities.

The server's `capabilities.textDocumentSync`: a number, or
`TextDocumentSyncOptions { change }`. Missing means `None` per the spec;
we keep `Full`, which every server understands.

# Examples

```rust
use crawrad_lsp::sync::{sync_kind_from, SyncKind};
use serde_json::json;

let caps = json!({ "textDocumentSync": 2 });
assert_eq!(sync_kind_from(&caps), SyncKind::Incremental);
```
