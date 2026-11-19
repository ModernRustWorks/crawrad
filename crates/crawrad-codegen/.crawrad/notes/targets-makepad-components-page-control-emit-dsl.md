---
id: targets-makepad-components-page-control-emit-dsl
kind: doc
target_file: src/targets/makepad/components/page_control.rs
line_hint: 354
author: 
created: 
title: "emit_dsl"
tags: []
signature: "pub fn emit_dsl (doc : & Document , reg : & crawrad_registry :: Registry , node : & Node , x : f32 , y : f32 , w : f32 , h : f32 ,) -> Result < String , crate :: Erro >"
---

Emits the `PageControl` DSL with the tabs header and integrated pages body.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::page_control::emit_dsl;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let reg = Registry::builtin();
let node = Node::new("pc", "PageControl");
let dsl = emit_dsl(&doc, &reg, &node, 0.0, 0.0, 200.0, 150.0).unwrap();
assert!(dsl.contains("pc = <View>"));
```

# Errors

Returns an error if an identifier name is invalid or child emission fails.
