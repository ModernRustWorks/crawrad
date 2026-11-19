---
id: layout-hash-bucket-mask
kind: doc
target_file: src/layout.rs
line_hint: 45
author: 
created: 
title: "HASH_BUCKET_MASK"
tags: []
signature: "pub const HASH_BUCKET_MASK : & [& str] = & [\"{}.base.table.table.bucket_mask\" , \"{}.base.map.table.table.bucket_mask\" ,]"
---

From a `HashMap`/`HashSet` to its number of buckets minus one.

# Examples

```rust
use crawrad_dap_rust::layout::HASH_BUCKET_MASK;

assert!(!HASH_BUCKET_MASK.is_empty());
```
