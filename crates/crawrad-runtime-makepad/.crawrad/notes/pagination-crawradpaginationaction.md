---
id: pagination-crawradpaginationaction
kind: doc
target_file: src/pagination.rs
line_hint: 148
author: 
created: 
title: "CrawradPaginationAction"
tags: []
signature: "pub enum CrawradPaginationAction"
---

What a [`CrawradPagination`] reports.

# Examples

```rust
use crawrad_runtime_makepad::pagination::CrawradPaginationAction;

let action = CrawradPaginationAction::Changed(1);
assert!(matches!(action, CrawradPaginationAction::Changed(1)));
```
