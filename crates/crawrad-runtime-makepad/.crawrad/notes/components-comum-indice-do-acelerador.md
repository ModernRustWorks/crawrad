---
id: components-comum-indice-do-acelerador
kind: doc
target_file: src/components/comum.rs
line_hint: 179
author: 
created: 
title: "indice_do_acelerador"
tags: []
signature: "pub fn indice_do_acelerador (t : & str) -> Option < usize >"
---

Byte where the underlined character begins, in the text without `&`.

Port of `indice_do_acelerador` from RXP
(`crawrad-runtime-egui/src/lib.rs:3359`). The index is in BYTES of the cleaned text,
to slice `&str` at no cost: the code that draws the underline measures the width of
prefix `[..i]` and of prefix `[..i + len_utf8]`, and the difference is the line.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::indice_do_acelerador;

assert_eq!(indice_do_acelerador("&Salvar"), Some(0));
assert_eq!(indice_do_acelerador("Salvar"), None);
```
