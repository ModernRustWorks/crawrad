---
id: dwarf-typeindex-from-types
kind: doc
target_file: src/dwarf.rs
line_hint: 117
author: 
created: 
title: "TypeIndex::from_types"
tags: []
signature: "pub fn from_types (types : Vec < Type >) -> Self"
---

An index built by hand — for tests of the presenter without a binary.

# Examples

```rust
use crawrad_dap_rust::dwarf::TypeIndex;

let index = TypeIndex::from_types(Vec::new());
assert!(index.is_empty());
```
