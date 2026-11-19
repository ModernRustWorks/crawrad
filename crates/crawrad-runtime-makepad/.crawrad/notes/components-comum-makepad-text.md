---
id: components-comum-makepad-text
kind: doc
target_file: src/components/comum.rs
line_hint: 88
author: 
created: 
title: "MAKEPAD_TEXT"
tags: []
signature: "pub const MAKEPAD_TEXT : [f32 ; 4] = crate :: shared_theme :: ROLE_TEXT"
---

O texto de um controle habilitado. Espelha o `CTL_TEXTO` do RXP.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_TEXT;

assert_eq!(MAKEPAD_TEXT[3], 1.0);
```
