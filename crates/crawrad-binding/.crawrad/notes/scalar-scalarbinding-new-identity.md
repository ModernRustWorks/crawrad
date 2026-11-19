---
id: scalar-scalarbinding-new-identity
kind: doc
target_file: src/scalar.rs
line_hint: 170
author: 
created: 
title: "ScalarBinding::new_identity"
tags: []
signature: "pub fn new_identity (source : Observable < T > , target : Observable < T > , mode : BindingMode) -> Self"
---

Convenience constructor for identical source and target types.

# Examples

```rust
use crawrad_binding::{BindingMode, Observable, ScalarBinding};

let binding = ScalarBinding::new_identity(Observable::new(1), Observable::new(0), BindingMode::OneWay);
assert_eq!(binding.mode(), BindingMode::OneWay);
```
