---
id: targets-egui-layout-emitido
kind: doc
target_file: src/targets/egui/layout.rs
line_hint: 15
author: 
created: 
title: "emitido"
tags: []
signature: "pub fn emitido (class : & str) -> bool"
---

Whether the class becomes code in the generated app (`emits_code`).

# Examples

```rust
use crawrad_codegen::targets::egui::layout::emitido;

assert!(emitido("Button"));
assert!(!emitido("GridColumn"));
```
