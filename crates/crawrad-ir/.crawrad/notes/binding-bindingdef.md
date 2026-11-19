---
id: binding-bindingdef
kind: doc
target_file: src/binding.rs
line_hint: 32
author: 
created: 
title: "BindingDef"
tags: []
signature: "pub struct BindingDef"
---

Declarative binding specification for a component property.

A declarative binding specification attached to a component property in `.crawform`.

# Examples

```rust
use crawrad_ir::{BindingDef, BindingMode};

let binding = BindingDef::new("ds_clients", BindingMode::TwoWay);
assert_eq!(binding.source, "ds_clients");
assert_eq!(binding.mode, BindingMode::TwoWay);
```
