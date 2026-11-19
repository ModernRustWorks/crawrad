---
id: targets-shared-dsl-colour
kind: doc
target_file: src/targets/shared.rs
line_hint: 1
author: 
created: 
title: "dsl_colour"
tags: []
signature: "pub fn dsl_colour (hex : & str) -> String"
---

A hex colour as Makepad's `live_design!` must spell it: `#x` before a digit.

The DSL goes through Rust's tokenizer: `#26262e` is read as the number
`26262` with an empty exponent `e` and the generated app does not compile
(found by the template matrix, #919). Makepad accepts `#x26262e` for exactly
this; colours starting with a letter stay as they are.

# Examples

```rust
use crawrad_codegen::targets::shared::dsl_colour;

assert_eq!(dsl_colour("#26262E"), "#x26262E");
assert_eq!(dsl_colour("#E0612B"), "#E0612B");
```
