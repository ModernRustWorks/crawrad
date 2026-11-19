---
id: binding-bindingmode
kind: doc
target_file: src/binding.rs
line_hint: 6
author: 
created: 
title: "BindingMode"
tags: []
signature: "pub enum BindingMode"
---

Direction and frequency of data binding synchronization.

# Examples

```rust
use crawrad_ir::BindingMode;

assert_eq!(BindingMode::default(), BindingMode::OneWay);
assert_eq!(BindingMode::TwoWay.as_str(), "two_way");
```
