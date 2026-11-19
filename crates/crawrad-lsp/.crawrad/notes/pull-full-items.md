---
id: pull-full-items
kind: doc
target_file: src/pull.rs
line_hint: 81
author: 
created: 
title: "full_items"
tags: []
signature: "pub fn full_items (answer : & Value) -> Option < Vec < Value > >"
---

Extracts diagnostic items from a full report response.

The items of a `full` report (`DocumentDiagnosticReport`); `None` for
`unchanged`, an error or anything else: what is kept stays.

# Examples

```rust
use crawrad_lsp::pull::full_items;
use serde_json::json;

let report = json!({ "kind": "full", "items": [] });
assert_eq!(full_items(&report), Some(Vec::new()));
```
