---
id: dwarf-typeindex-member-path
kind: doc
target_file: src/dwarf.rs
line_hint: 148
author: 
created: 
title: "TypeIndex::member_path"
tags: []
signature: "pub fn member_path (& self , mut ty : TypeId , path : & [& str]) -> Option < (u64 , TypeId) >"
---

The offset and type of the member at `path` (`["vec", "len"]`) inside `ty`.

# Examples

```rust
use crawrad_dap_rust::dwarf::{Kind, Type, TypeIndex};

let ty = Type {
    name: "Test".into(),
    size: 8,
    kind: Kind::Struct { members: Vec::new(), variants: None },
};
let index = TypeIndex::from_types(vec![ty]);
assert!(index.member_path(0, &["nonexistent"]).is_none());
```
