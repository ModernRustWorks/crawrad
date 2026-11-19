---
id: scalar-scalarbinding-target
kind: doc
target_file: src/scalar.rs
line_hint: 52
author: 
created: 
title: "ScalarBinding::target"
tags: []
signature: "pub fn target (& self) -> & Observable < T >"
---

Access the underlying target observable.

# Examples

```rust
use crawrad_binding::{BindingMode, Observable, ScalarBinding};

let target = Observable::new(99);
let binding = ScalarBinding::new_identity(Observable::new(0), target, BindingMode::OneWay);
assert_eq!(binding.target().get(), 99);
```
