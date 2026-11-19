---
id: inspector-model-rows-json
kind: doc
target_file: src/inspector_model.rs
line_hint: 207
author: 
created: 
title: "rows_json"
tags: []
signature: "pub fn rows_json (rows : & [Row]) -> serde_json :: Value"
---

Serializes the rows of the Object Inspector to JSON.

The rows of the Object Inspector as `view.get_inspector` reports them (#556):
what each row shows, cell by cell, in the order on screen — so an agent reads
the panel a person sees, sub-rows of a folded property included.

# Examples

```rust
use crawrad_actions::inspector_model::{rows_json, Row};

let json = rows_json(&[Row::Notice("test")]);
assert!(json["rows"].is_array());
```
