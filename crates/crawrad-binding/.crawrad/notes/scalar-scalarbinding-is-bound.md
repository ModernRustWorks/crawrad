---
id: scalar-scalarbinding-is-bound
kind: doc
target_file: src/scalar.rs
line_hint: 42
author: 
created: 
title: "ScalarBinding::is_bound"
tags: []
signature: "pub fn is_bound (& self) -> bool"
---

Check if the binding is currently active.

# Examples

```rust
use crawrad_binding::{BindingMode, Observable, ScalarBinding};

let binding = ScalarBinding::new_identity(Observable::new(1), Observable::new(0), BindingMode::OneWay);
assert!(!binding.is_bound());
```
