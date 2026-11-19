---
id: scalar-scalarbinding-bind
kind: doc
target_file: src/scalar.rs
line_hint: 113
author: 
created: 
title: "ScalarBinding::bind"
tags: []
signature: "pub fn bind (& self) -> Result < () , String >"
---

Activates the binding according to `BindingMode`.

Activate the binding and setup reactive subscriptions according to `BindingMode`.

# Examples

```rust
use crawrad_binding::{BindingMode, Observable, ScalarBinding};

let source = Observable::new(5);
let target = Observable::new(0);
let binding = ScalarBinding::new_identity(source, target.clone(), BindingMode::OneWay);
assert!(binding.bind().is_ok());
assert_eq!(target.get(), 5);
```

# Errors

Returns an error if initial synchronization fails during conversion.

# Panics

Panics if the internal subscription mutex is poisoned.
