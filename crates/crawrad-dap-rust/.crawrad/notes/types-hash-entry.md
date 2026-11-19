---
id: types-hash-entry
kind: doc
target_file: src/types.rs
line_hint: 175
author: 
created: 
title: "hash_entry"
tags: []
signature: "pub fn hash_entry (name : & str) -> Option < (String , bool) >"
---

Returns the entry type name and whether the container is a set.

The DWARF name of a `HashMap`'s or `HashSet`'s entry, and whether it is a
set: `HashMap<alloc::string::String, unsigned int, …>` →
`("(alloc::string::String, u32)", false)`; `HashSet<int, …>` → `("(i32, ())", true)`.
hashbrown stores a map's `(K, V)` and a set's `(T, ())` in its buckets.

# Examples

```rust
use crawrad_dap_rust::types::hash_entry;

let res = hash_entry("std::collections::hash::map::HashMap<int, int>");
assert_eq!(res, Some(("(i32, i32)".to_string(), false)));
```
