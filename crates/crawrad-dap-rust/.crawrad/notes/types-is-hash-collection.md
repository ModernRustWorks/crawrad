---
id: types-is-hash-collection
kind: doc
target_file: src/types.rs
line_hint: 169
author: 
created: 
title: "is_hash_collection"
tags: []
signature: "pub fn is_hash_collection (name : & str) -> bool"
---

`HashMap` or `HashSet` of the standard library.

# Examples

```rust
use crawrad_dap_rust::types::is_hash_collection;

assert!(is_hash_collection("std::collections::hash::map::HashMap<i32, i32>"));
```
