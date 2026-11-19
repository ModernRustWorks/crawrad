---
id: dwarf-typeindex-iter
kind: doc
target_file: src/dwarf.rs
line_hint: 135
author: 
created: 
title: "TypeIndex::iter"
tags: []
signature: "pub fn iter (& self) -> impl Iterator < Item = (TypeId , & Type) >"
---

Every type, with its id.

# Examples

```rust
use crawrad_dap_rust::dwarf::TypeIndex;

let index = TypeIndex::from_types(Vec::new());
assert_eq!(index.iter().count(), 0);
```
