---
id: scalar-scalarbinding-source
kind: doc
target_file: src/scalar.rs
line_hint: 47
author: 
created: 
title: "ScalarBinding::source"
tags: []
signature: "pub fn source (& self) -> & Observable < S >"
---

Access the underlying source observable.

# Examples

```rust
use crawrad_binding::{BindingMode, Observable, ScalarBinding};

let source = Observable::new(42);
let binding = ScalarBinding::new_identity(source, Observable::new(0), BindingMode::OneWay);
assert_eq!(binding.source().get(), 42);
```
