---
id: binding-bindingdef-with-field
kind: doc
target_file: src/binding.rs
line_hint: 59
author: 
created: 
title: "BindingDef::with_field"
tags: []
signature: "pub fn with_field (mut self , field : impl Into < String >) -> Self"
---

Set the target column/field name for scalar property binding.

# Examples

```rust
use crawrad_ir::{BindingDef, BindingMode};

let binding = BindingDef::new("ds_orders", BindingMode::OneWay).with_field("total");
assert_eq!(binding.field.as_deref(), Some("total"));
```
