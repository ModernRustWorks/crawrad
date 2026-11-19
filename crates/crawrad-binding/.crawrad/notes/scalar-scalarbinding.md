---
id: scalar-scalarbinding
kind: doc
target_file: src/scalar.rs
line_hint: 11
author: 
created: 
title: "ScalarBinding"
tags: []
signature: "pub struct ScalarBinding < S , T , C = IdentityConverter < S > >"
---

Synchronizes a source property and a target visual control property.

Manages the synchronization between a source property and a target visual control property.

# Examples

```rust
use crawrad_binding::{BindingMode, Observable, ScalarBinding};

let source = Observable::new(1);
let target = Observable::new(0);
let binding = ScalarBinding::new_identity(source, target, BindingMode::OneWay);
assert!(!binding.is_bound());
```
