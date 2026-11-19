---
id: targets-alvogerador
kind: doc
target_file: src/targets/mod.rs
line_hint: 20
author: 
created: 
title: "AlvoGerador"
tags: []
signature: "pub enum AlvoGerador"
---

Identifier of the code generation target for the final application.

# Examples

```rust
use crawrad_codegen::targets::AlvoGerador;

assert_eq!(AlvoGerador::default(), AlvoGerador::Makepad);
```
