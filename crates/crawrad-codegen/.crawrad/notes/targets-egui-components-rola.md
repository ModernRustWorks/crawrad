---
id: targets-egui-components-rola
kind: doc
target_file: src/targets/egui/components/mod.rs
line_hint: 198
author: 
created: 
title: "rola"
tags: []
signature: "pub fn rola (classe : & str) -> bool"
---

Checks whether the class opens a ScrollArea in the generated code.

Does this class open an `egui::ScrollArea` in the generated code?

# Examples

```rust
use crawrad_codegen::targets::egui::components::rola;

assert!(!rola("Button"));
```
