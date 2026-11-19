---
id: components-category-panel-category-panel
kind: doc
target_file: src/components/category_panel.rs
line_hint: 5
author: 
created: 
title: "category_panel"
tags: []
signature: "pub fn category_panel () -> ComponentDesc"
---

A collapsible container panel under a header.

`CategoryPanel` (#500, wave 1): a container under a header; a click on the
header folds it to the header alone (hiding its children) and back.

# Examples

```rust
use crawrad_registry::category_panel;

let desc = category_panel();
assert_eq!(desc.class, "CategoryPanel");
```
