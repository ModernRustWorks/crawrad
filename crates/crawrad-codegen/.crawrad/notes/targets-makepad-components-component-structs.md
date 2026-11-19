---
id: targets-makepad-components-component-structs
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 166
author: 
created: 
title: "component_structs"
tags: []
signature: "pub fn component_structs (doc : & Document , reg : & crawrad_registry :: Registry) -> String"
---

Aggregates component struct definitions for controls used in the form.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::component_structs;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let reg = Registry::builtin();
let structs = component_structs(&doc, &reg);
assert!(!structs.is_empty());
```
