---
id: components-list-view-list-column
kind: doc
target_file: src/components/list_view.rs
line_hint: 36
author: 
created: 
title: "list_column"
tags: []
signature: "pub fn list_column () -> ComponentDesc"
---

One column of a `ListView` (#500): caption, width in points and text alignment.

# Examples

```rust
use crawrad_registry::list_column;

let desc = list_column();
assert_eq!(desc.class, "ListColumn");
```
