---
id: memory-from-bytes
kind: doc
target_file: src/memory.rs
line_hint: 43
author: 
created: 
title: "from_bytes"
tags: []
signature: "pub fn from_bytes (index : & TypeIndex , ty : TypeId , bytes : & [u8] , addr : u64 , src : & dyn ValueSource , depth : usize ,) -> Option < String >"
---

`bytes` hold one value of type `ty`, which lives at `addr`.

# Examples

```rust
use crawrad_dap::DapConnection;
use crawrad_dap_rust::dwarf::{BaseEncoding, Kind, Type, TypeIndex};
use crawrad_dap_rust::memory::from_bytes;

let ty = Type { name: "u32".into(), size: 4, kind: Kind::Base(BaseEncoding::Unsigned) };
let index = TypeIndex::from_types(vec![ty]);
let conn = DapConnection::new(std::io::empty(), std::io::sink());
let val = from_bytes(&index, 0, &[42, 0, 0, 0], 0x1000, &conn, 1);
assert_eq!(val, Some("42".into()));
```
