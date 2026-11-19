---
id: layout-hash-ctrl
kind: doc
target_file: src/layout.rs
line_hint: 38
author: 
created: 
title: "HASH_CTRL"
tags: []
signature: "pub const HASH_CTRL : & [& str] = & [\"{}.base.table.table.ctrl.pointer\" , \"{}.base.map.table.table.ctrl.pointer\" ,]"
---

Path expressions from a hash collection to its control bytes.

From a `HashMap`/`HashSet` to its control bytes; its buckets sit just below
them, bucket `i` at `ctrl - (i + 1) * size_of::<entry>()` (hashbrown).

# Examples

```rust
use crawrad_dap_rust::layout::HASH_CTRL;

assert!(!HASH_CTRL.is_empty());
```
