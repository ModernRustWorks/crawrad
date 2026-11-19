---
id: binding-bindingdef-new
kind: doc
target_file: src/binding.rs
line_hint: 49
author: 
created: 
title: "BindingDef::new"
tags: []
signature: "pub fn new (source : impl Into < String > , mode : BindingMode) -> Self"
---

Create a new binding targeting a data source with the specified mode.

# Examples

```rust
use crawrad_ir::{BindingDef, BindingMode};

let binding = BindingDef::new("ds_orders", BindingMode::OneWay);
assert_eq!(binding.source, "ds_orders");
assert_eq!(binding.mode, BindingMode::OneWay);
assert_eq!(binding.field, None);
```
