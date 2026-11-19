---
id: sem-acelerador
kind: doc
target_file: src/lib.rs
line_hint: 896
author: 
created: 
title: "sem_acelerador"
tags: []
signature: "pub fn sem_acelerador (t : & str) -> String"
---

Removes accelerator markers (&) from text for display.

The classic accelerator `&` does not go into the generated code: `&&` is a literal `&`, and a single `&` marks the underlined letter.

# Examples

```rust
use crawrad_codegen::sem_acelerador;

assert_eq!(sem_acelerador("&Salvar"), "Salvar");
assert_eq!(sem_acelerador("Copiar && Colar"), "Copiar & Colar");
```
