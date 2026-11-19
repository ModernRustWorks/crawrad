---
id: identificador
kind: doc
target_file: src/lib.rs
line_hint: 829
author: 
created: 
title: "identificador"
tags: []
signature: "pub fn identificador (nome : & str) -> Result < String , Erro >"
---

Validates and returns a valid Rust identifier for a component name.

The component's `Name` becomes a field and method in generated code, so it
must be a valid Rust identifier.

# Examples

```rust
use crawrad_codegen::identificador;

assert_eq!(identificador("btn_ok").unwrap(), "btn_ok");
assert!(identificador("type").is_err());
```

# Errors

Returns an error if the name is empty, starts with a digit, contains invalid characters,
or is a reserved Rust keyword.

This is the reason `Name` remains unique and validated after
ADR 0008: the `id` is
stable and opaque, but what turns into code is the name.
