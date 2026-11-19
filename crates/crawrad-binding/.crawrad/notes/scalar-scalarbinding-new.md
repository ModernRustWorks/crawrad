---
id: scalar-scalarbinding-new
kind: doc
target_file: src/scalar.rs
line_hint: 79
author: 
created: 
title: "ScalarBinding::new"
tags: []
signature: "pub fn new (source : Observable < S > , target : Observable < T > , mode : BindingMode , converter : C ,) -> Self"
---

Create a new scalar binding with a custom converter.

# Examples

```rust
use crawrad_binding::{BindingMode, IdentityConverter, Observable, ScalarBinding};

let binding = ScalarBinding::new(
    Observable::new(1),
    Observable::new(0),
    BindingMode::OneWay,
    IdentityConverter::new(),
);
assert_eq!(binding.mode(), BindingMode::OneWay);
```
