---
id: converter-valueconverter
kind: doc
target_file: src/converter.rs
line_hint: 8
author: 
created: 
title: "ValueConverter"
tags: []
signature: "pub trait ValueConverter < S , T > : Send + Sync + 'static"
---

Bidirectional converter between source type `S` and target type `T`.

# Examples

```rust
use crawrad_binding::{IdentityConverter, ValueConverter};

let conv = IdentityConverter::<i32>::new();
assert_eq!(conv.convert(&42), Ok(42));
```
