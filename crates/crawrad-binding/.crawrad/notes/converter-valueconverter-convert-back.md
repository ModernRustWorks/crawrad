---
id: converter-valueconverter-convert-back
kind: doc
target_file: src/converter.rs
line_hint: 13
author: 
created: 
title: "ValueConverter::convert_back"
tags: []
signature: "fn convert_back (& self , target : & T) -> Result < S , String >"
---

Convert from target value back to source value.

# Examples

```rust
use crawrad_binding::{IdentityConverter, ValueConverter};

let conv = IdentityConverter::<i32>::new();
assert_eq!(conv.convert_back(&7), Ok(7));
```

# Errors

Returns an error if the conversion from target back to source fails.
