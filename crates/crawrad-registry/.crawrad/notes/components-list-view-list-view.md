---
id: components-list-view-list-view
kind: doc
target_file: src/components/list_view.rs
line_hint: 7
author: 
created: 
title: "list_view"
tags: []
signature: "pub fn list_view () -> ComponentDesc"
---

A report-style list view with column headers and virtualized rows.

`ListView`: report-style list with column headers, virtualized rows, captions and sub-items (#500).

# Examples

```rust
use crawrad_registry::list_view;

let desc = list_view();
assert_eq!(desc.class, "ListView");
```
