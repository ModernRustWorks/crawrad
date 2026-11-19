---
id: memory-vec-at
kind: doc
target_file: src/memory.rs
line_hint: 190
author: 
created: 
title: "vec_at"
tags: []
signature: "pub fn vec_at (index : & TypeIndex , elem : & str , ptr : u64 , len : u64 , src : & dyn ValueSource , depth : usize ,) -> Option < String >"
---

The elements of a `Vec<elem>` of `len` items at `ptr`.

# Examples

```rust
use crawrad_dap::DapConnection;
use crawrad_dap_rust::dwarf::TypeIndex;
use crawrad_dap_rust::memory::vec_at;

let index = TypeIndex::from_types(Vec::new());
let conn = DapConnection::new(std::io::empty(), std::io::sink());
let val = vec_at(&index, "u32", 0x1000, 0, &conn, 1);
assert_eq!(val, Some("[]".into()));
```
