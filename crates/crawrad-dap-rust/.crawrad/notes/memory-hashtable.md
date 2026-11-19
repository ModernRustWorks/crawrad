---
id: memory-hashtable
kind: doc
target_file: src/memory.rs
line_hint: 241
author: 
created: 
title: "HashTable"
tags: []
signature: "pub struct HashTable"
---

Where a hashbrown table keeps its entries.

# Examples

```rust
use crawrad_dap_rust::memory::HashTable;

let table = HashTable { ctrl: 0x1000, buckets: 16, items: 3 };
assert_eq!(table.items, 3);
```
