---
id: components-pagination-pagination
kind: doc
target_file: src/components/pagination.rs
line_hint: 4
author: 
created: 
title: "pagination"
tags: []
signature: "pub fn pagination () -> ComponentDesc"
---

`Pagination`: a row of page buttons (#500).

# Examples

```rust
use crawrad_registry::pagination;

let desc = pagination();
assert_eq!(desc.class, "Pagination");
```
