---
id: list-view-listviewcolumns
kind: doc
target_file: src/list_view.rs
line_hint: 123
author: 
created: 
title: "ListViewColumns"
tags: []
signature: "pub type ListViewColumns = Vec < ListViewColumn >"
---

Collection of columns.

# Examples

```rust
use crawrad_runtime_makepad::list_view::ListViewColumns;

let cols: ListViewColumns = Vec::new();
assert!(cols.is_empty());
```
