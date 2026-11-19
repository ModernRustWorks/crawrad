---
id: scalar-scalarbinding-mode
kind: doc
target_file: src/scalar.rs
line_hint: 37
author: 
created: 
title: "ScalarBinding::mode"
tags: []
signature: "pub fn mode (& self) -> BindingMode"
---

Access the binding mode.

# Examples

```rust
use crawrad_binding::{BindingMode, Observable, ScalarBinding};

let binding = ScalarBinding::new_identity(Observable::new(1), Observable::new(0), BindingMode::OneWay);
assert_eq!(binding.mode(), BindingMode::OneWay);
```
