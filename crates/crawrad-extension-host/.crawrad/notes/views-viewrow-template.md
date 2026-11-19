---
id: views-viewrow-template
kind: doc
target_file: src/views.rs
line_hint: 142
author: 
created: 
title: "ViewRow::template"
tags: []
signature: "pub fn template (& self) -> & 'static str"
---

The list's template for the row.

# Examples

```rust
use crawrad_extension_host::views::ViewRow;

let row = ViewRow::Section { title: "Installed".into(), count: Some(1) };
assert_eq!(row.template(), "ViewHeader");
```
