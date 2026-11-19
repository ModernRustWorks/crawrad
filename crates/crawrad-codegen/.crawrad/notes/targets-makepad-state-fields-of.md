---
id: targets-makepad-state-fields-of
kind: doc
target_file: src/targets/makepad/state.rs
line_hint: 216
author: 
created: 
title: "fields_of"
tags: []
signature: "pub fn fields_of (desc : & ComponentDesc) -> Vec < (String , String) >"
---

Os campos do estado de uma classe, como `(nome, tipo)`.

**Os campos do estado de uma classe**, como `(nome, tipo)`.

Derivados do descritor, e de mais nada.

# Examples

```rust
use crawrad_codegen::targets::makepad::state::fields_of;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let fields = fields_of(desc);
assert!(!fields.is_empty());
```
