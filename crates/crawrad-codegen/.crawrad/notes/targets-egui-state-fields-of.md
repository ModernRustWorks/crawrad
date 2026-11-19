---
id: targets-egui-state-fields-of
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 458
author: 
created: 
title: "fields_of"
tags: []
signature: "pub fn fields_of (desc : & ComponentDesc) -> Vec < (String , String) >"
---

Computes the list of `(name, type)` fields for a component descriptor.

# Examples

```rust
use crawrad_codegen::targets::egui::state::fields_of;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let fields = fields_of(desc);
assert!(!fields.is_empty());
```
