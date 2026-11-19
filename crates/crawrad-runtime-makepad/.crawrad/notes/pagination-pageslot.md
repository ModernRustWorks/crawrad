---
id: pagination-pageslot
kind: doc
target_file: src/pagination.rs
line_hint: 60
author: 
created: 
title: "PageSlot"
tags: []
signature: "pub enum PageSlot"
---

A slot in a pagination control (#500).

# Examples

```rust
use crawrad_runtime_makepad::pagination::PageSlot;

let slot = PageSlot::Page(0);
assert_eq!(slot, PageSlot::Page(0));
```
