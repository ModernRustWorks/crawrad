---
id: memory-hash-at
kind: doc
target_file: src/memory.rs
line_hint: 252
author: 
created: 
title: "hash_at"
tags: []
signature: "pub fn hash_at (index : & TypeIndex , entry : & str , set : bool , table : & HashTable , src : & dyn ValueSource , depth : usize ,) -> Option < String >"
---

Formats the entries of a HashMap or HashSet from raw memory.

The entries of a `HashMap` (`{k: v, …}`) or `HashSet` (`{k, …}`) whose
buckets hold `entry` — `(K, V)`, or `(T, ())` for a set — in the order
hashbrown iterates them, which is the order `{:?}` prints.

# Examples

```rust
use crawrad_dap::DapConnection;
use crawrad_dap_rust::dwarf::TypeIndex;
use crawrad_dap_rust::memory::{HashTable, hash_at};

let index = TypeIndex::from_types(Vec::new());
let conn = DapConnection::new(std::io::empty(), std::io::sink());
let table = HashTable { ctrl: 0, buckets: 8, items: 0 };
let val = hash_at(&index, "(i32, i32)", false, &table, &conn, 1);
assert_eq!(val, Some("{}".into()));
```
