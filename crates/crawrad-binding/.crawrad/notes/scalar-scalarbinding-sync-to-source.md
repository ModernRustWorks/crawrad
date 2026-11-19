---
id: scalar-scalarbinding-sync-to-source
kind: doc
target_file: src/scalar.rs
line_hint: 105
author: 
created: 
title: "ScalarBinding::sync_to_source"
tags: []
signature: "pub fn sync_to_source (& self) -> Result < () , String >"
---

Synchronize the value from target to source immediately.

# Examples

```rust
use crawrad_binding::{BindingMode, Observable, ScalarBinding};

let source = Observable::new(0);
let target = Observable::new(20);
let binding = ScalarBinding::new_identity(source.clone(), target, BindingMode::OneWay);
assert!(binding.sync_to_source().is_ok());
assert_eq!(source.get(), 20);
```

# Errors

Returns an error if the converter fails to convert the target value back.
