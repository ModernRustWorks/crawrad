---
id: converter-identityconverter
kind: doc
target_file: src/converter.rs
line_hint: 17
author: 
created: 
title: "IdentityConverter"
tags: []
signature: "pub struct IdentityConverter < T >"
---

Direct identity converter for matching types `T` -> `T`.

# Examples

```rust
use crawrad_binding::{IdentityConverter, ValueConverter};

let conv = IdentityConverter::<i32>::new();
assert_eq!(conv.convert(&42), Ok(42));
```
