---
id: item-known-detail
kind: doc
target_file: src/item.rs
line_hint: 353
author: 
created: 
title: "known_detail"
tags: []
signature: "pub fn known_detail (name : & str) -> Option < & 'static str >"
---

The signature shown beside a name the local list knows without a server.

# Examples

```rust
use crawrad_insight::item::known_detail;

assert_eq!(known_detail("len"), Some("fn(&self) -> usize"));
```
