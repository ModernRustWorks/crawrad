---
id: scalar-scalarbinding-unbind
kind: doc
target_file: src/scalar.rs
line_hint: 57
author: 
created: 
title: "ScalarBinding::unbind"
tags: []
signature: "pub fn unbind (& self)"
---

Deactivate the binding and unsubscribe all listeners.

# Examples

```rust
use crawrad_binding::{BindingMode, Observable, ScalarBinding};

let binding = ScalarBinding::new_identity(Observable::new(1), Observable::new(0), BindingMode::OneWay);
binding.bind().unwrap();
binding.unbind();
assert!(!binding.is_bound());
```

# Panics

Panics if the internal subscription mutex is poisoned.
