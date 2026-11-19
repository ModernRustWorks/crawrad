---
id: components-comum-sem-acelerador
kind: doc
target_file: src/components/comum.rs
line_hint: 162
author: 
created: 
title: "sem_acelerador"
tags: []
signature: "pub fn sem_acelerador (t : & str) -> String"
---

`&&` is a literal `&`; a lone `&` marks the accelerator and is not drawn.

Literal port of `sem_acelerador` from RXP
(`crawrad-runtime-egui/src/lib.rs:3342`). It lives HERE, rather than in each component
file, because the rule comes from a classic RAD IDE and applies to every caption — `Button`,
`CheckBox`, `RadioButton`, `GroupBox`, `Label`, and the tab of `PageControl`.
Without it, the button displayed **`&OK`** on screen, with the `&` written out literally.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::sem_acelerador;

assert_eq!(sem_acelerador("&Salvar"), "Salvar");
assert_eq!(sem_acelerador("P&&D"), "P&D");
```
