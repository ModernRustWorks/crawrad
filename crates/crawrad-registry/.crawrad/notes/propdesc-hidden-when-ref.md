---
id: propdesc-hidden-when-ref
kind: doc
target_file: src/lib.rs
line_hint: 206
author: 
created: 
title: "PropDesc::hidden_when_ref"
tags: []
signature: "pub fn hidden_when_ref (mut self , reference : & 'static str , capability : & 'static str) -> Self"
---

Hides the property while a sibling reference points at a given capability.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::table_name("").hidden_when_ref("data_source", "DbDataset");
assert_eq!(p.hidden_when, Some(("data_source", "DbDataset")));
```
