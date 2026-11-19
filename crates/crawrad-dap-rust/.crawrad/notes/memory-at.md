---
id: memory-at
kind: doc
target_file: src/memory.rs
line_hint: 15
author: 
created: 
title: "at"
tags: []
signature: "pub fn at (index : & TypeIndex , ty : TypeId , addr : u64 , src : & dyn ValueSource , depth : usize ,) -> Option < String >"
---

The value of type `ty` stored at `addr`, as `{:?}` prints it.

# Examples

```rust
use crawrad_dap::DapConnection;
use crawrad_dap_rust::dwarf::{Kind, Type, TypeIndex};
use crawrad_dap_rust::memory::at;

let ty = Type { name: "Unit".into(), size: 0, kind: Kind::Other };
let index = TypeIndex::from_types(vec![ty]);
let conn = DapConnection::new(std::io::empty(), std::io::sink());
let val = at(&index, 0, 0x1000, &conn, 1);
assert_eq!(val, Some("Unit".into()));
```
