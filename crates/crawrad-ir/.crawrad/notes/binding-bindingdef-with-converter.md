---
id: binding-bindingdef-with-converter
kind: doc
target_file: src/binding.rs
line_hint: 65
author: 
created: 
title: "BindingDef::with_converter"
tags: []
signature: "pub fn with_converter (mut self , converter : impl Into < String >) -> Self"
---

Set an optional converter name.

# Examples

```rust
use crawrad_ir::{BindingDef, BindingMode};

let binding = BindingDef::new("ds_orders", BindingMode::OneWay).with_converter("currency");
assert_eq!(binding.converter.as_deref(), Some("currency"));
```
