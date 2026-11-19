---
id: targets-egui-state-struct-of
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 484
author: 
created: 
title: "struct_of"
tags: []
signature: "pub fn struct_of (desc : & ComponentDesc) -> String"
---

Emits the `pub struct <Class> { ... }` code for a component descriptor.

# Examples

```rust
use crawrad_codegen::targets::egui::state::struct_of;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let s = struct_of(desc);
assert!(s.contains("pub struct Button"));
```
