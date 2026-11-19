---
id: converter-valueconverter-convert
kind: doc
target_file: src/converter.rs
line_hint: 10
author: 
created: 
title: "ValueConverter::convert"
tags: []
signature: "fn convert (& self , source : & S) -> Result < T , String >"
---

Convert from source value to target value.

# Examples

```rust
use crawrad_binding::{IdentityConverter, ValueConverter};

let conv = IdentityConverter::<i32>::new();
assert_eq!(conv.convert(&7), Ok(7));
```

# Errors

Returns an error if the conversion from source to target fails.
