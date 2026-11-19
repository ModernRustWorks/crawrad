---
id: targets-makepad-layout-is-inside-container
kind: doc
target_file: src/targets/makepad/layout.rs
line_hint: 107
author: 
created: 
title: "is_inside_container"
tags: []
signature: "pub fn is_inside_container (doc : & Document , reg : & crawrad_registry :: Registry , id : & str) -> bool"
---

Returns true if the node is inside any visual container.

Returns true if the node is inside any visual container (Panel, GroupBox, ScrollBox, PageControl, TabSheet).

# Examples

```rust
use crawrad_codegen::targets::makepad::layout::is_inside_container;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("form", "Form");
assert!(!is_inside_container(&doc, &reg, "btn"));
```
