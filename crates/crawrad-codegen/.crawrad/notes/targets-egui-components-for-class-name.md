---
id: targets-egui-components-for-class-name
kind: doc
target_file: src/targets/egui/components/mod.rs
line_hint: 163
author: 
created: 
title: "for_class_name"
tags: []
signature: "pub fn for_class_name (name : & str) -> Option < & 'static dyn EguiComponentCodegen >"
---

The generator for this class, if this target registered one.

`None` for a class that this target does not emit — and what decides what to do
with it is `emitir_componente`, in a single place.

# Examples

```rust
use crawrad_codegen::targets::egui::components::for_class_name;

let codegen = for_class_name("Button");
assert!(codegen.is_some());
```
