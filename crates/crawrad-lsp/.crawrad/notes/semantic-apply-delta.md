---
id: semantic-apply-delta
kind: doc
target_file: src/semantic.rs
line_hint: 146
author: 
created: 
title: "apply_delta"
tags: []
signature: "pub fn apply_delta (prev : & [u32] , edits : & [Value]) -> Option < Vec < u32 > >"
---

Applies semantic token delta edits to previous token data.

The previous `data` with a delta's `edits` applied (`start`,
`deleteCount`, `data`), from the last edit to the first.

# Examples

```rust
use crawrad_lsp::semantic::apply_delta;
use serde_json::json;

let prev = vec![1, 2, 3];
let edits = vec![json!({ "start": 1, "deleteCount": 1, "data": [9, 8] })];
assert_eq!(apply_delta(&prev, &edits), Some(vec![1, 9, 8, 3]));
```
