---
id: views-viewrow
kind: doc
target_file: src/views.rs
line_hint: 109
author: 
created: 
title: "ViewRow"
tags: []
signature: "pub enum ViewRow"
---

A row of the Extensions panel's list.

# Examples

```rust
use crawrad_extension_host::views::ViewRow;

let row = ViewRow::Section { title: "Installed".into(), count: Some(1) };
assert_eq!(row.template(), "ViewHeader");
```
