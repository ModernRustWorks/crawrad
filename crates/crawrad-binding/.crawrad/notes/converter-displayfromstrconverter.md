---
id: converter-displayfromstrconverter
kind: doc
target_file: src/converter.rs
line_hint: 41
author: 
created: 
title: "DisplayFromStrConverter"
tags: []
signature: "pub struct DisplayFromStrConverter < S >"
---

Bidirectional converter between a `Display + FromStr` type and `String`.

Bidirectional converter between any `Display + FromStr` type and `String`.
E.g. `i32` <-> `String`, `f64` <-> `String`.

# Examples

```rust
use crawrad_binding::{DisplayFromStrConverter, ValueConverter};

let conv = DisplayFromStrConverter::<i32>::new();
assert_eq!(conv.convert(&42), Ok("42".to_string()));
assert_eq!(conv.convert_back(&"42".to_string()), Ok(42));
```
