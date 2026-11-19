---
id: targets-makepad-binding-binding-declarado
kind: doc
target_file: src/targets/makepad/binding.rs
line_hint: 290
author: 
created: 
title: "binding_declarado"
tags: []
signature: "pub fn binding_declarado (event : & str) -> Option < Binding >"
---

The binding of a declared component for this event, if any.

# Examples

```rust
use crawrad_codegen::targets::makepad::binding::binding_declarado;

let b = binding_declarado("on_click");
assert!(b.is_some());
```
