---
id: form-formbase-new
kind: doc
target_file: src/form.rs
line_hint: 35
author: 
created: 
title: "FormBase::new"
tags: []
signature: "pub fn new (title : impl Into < String > , width : f32 , height : f32) -> Self"
---

Creates a new `FormBase` with the specified title and dimensions.

# Examples

```rust
use crawrad_runtime::FormBase;

let form = FormBase::new("Main Form", 800.0, 600.0);
assert_eq!(form.title, "Main Form");
assert_eq!(form.width, 800.0);
```
