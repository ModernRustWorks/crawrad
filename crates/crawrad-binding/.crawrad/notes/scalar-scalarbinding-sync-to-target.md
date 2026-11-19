---
id: scalar-scalarbinding-sync-to-target
kind: doc
target_file: src/scalar.rs
line_hint: 97
author: 
created: 
title: "ScalarBinding::sync_to_target"
tags: []
signature: "pub fn sync_to_target (& self) -> Result < () , String >"
---

Synchronize the value from source to target immediately.

# Examples

```rust
use crawrad_binding::{BindingMode, Observable, ScalarBinding};

let source = Observable::new(10);
let target = Observable::new(0);
let binding = ScalarBinding::new_identity(source, target.clone(), BindingMode::OneWay);
assert!(binding.sync_to_target().is_ok());
assert_eq!(target.get(), 10);
```

# Errors

Returns an error if the converter fails to convert the source value.
