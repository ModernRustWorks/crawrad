---
id: layout-hash-len
kind: doc
target_file: src/layout.rs
line_hint: 30
author: 
created: 
title: "HASH_LEN"
tags: []
signature: "pub const HASH_LEN : & [& str] = & [\"{}.base.table.table.items\" , \"{}.base.map.table.table.items\" ,]"
---

From a `HashMap`/`HashSet` to its number of items (hashbrown's `items`).

# Examples

```rust
use crawrad_dap_rust::layout::HASH_LEN;

assert!(!HASH_LEN.is_empty());
```
