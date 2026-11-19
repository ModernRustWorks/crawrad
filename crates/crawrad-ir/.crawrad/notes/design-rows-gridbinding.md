---
id: design-rows-gridbinding
kind: doc
target_file: src/design_rows.rs
line_hint: 126
author: 
created: 
title: "GridBinding"
tags: []
signature: "pub struct GridBinding < 'a >"
---

Where a bound Grid reads from: its connection, and the dataset in between.

# Examples

```rust
use crawrad_ir::{Document, Node};
use crawrad_ir::design_rows::GridBinding;

let conn = Node::new("conn1", "Connection");
let b = GridBinding { connection: &conn, dataset: None };
assert!(b.dataset.is_none());
let _ = Document::new("form_main", "Form");
```
