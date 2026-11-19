---
id: converter-closureconverter
kind: doc
target_file: src/converter.rs
line_hint: 69
author: 
created: 
title: "ClosureConverter"
tags: []
signature: "pub struct ClosureConverter < S , T , F , B > where F : Fn (& S) -> Result < T , String > + Send + Sync + 'static , B : Fn (& T) -> Result < S , String > + Send + Sync + 'static ,"
---

Flexible converter created from forward and backward closures.

# Examples

```rust
use crawrad_binding::{ClosureConverter, ValueConverter};

let conv = ClosureConverter::new(|n: &i32| Ok(n * 2), |n: &i32| Ok(n / 2));
assert_eq!(conv.convert(&10), Ok(20));
assert_eq!(conv.convert_back(&20), Ok(10));
```
