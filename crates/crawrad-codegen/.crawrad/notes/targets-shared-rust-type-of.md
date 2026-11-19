---
id: targets-shared-rust-type-of
kind: doc
target_file: src/targets/shared.rs
line_hint: 501
author: 
created: 
title: "rust_type_of"
tags: []
signature: "pub fn rust_type_of (default : & crawrad_ir :: Value) -> & 'static str"
---

The Rust type that a default value implies across code generators.

# Examples

```rust
use crawrad_codegen::targets::shared::rust_type_of;
use crawrad_ir::Value;

assert_eq!(rust_type_of(&Value::Bool(true)), "bool");
```
