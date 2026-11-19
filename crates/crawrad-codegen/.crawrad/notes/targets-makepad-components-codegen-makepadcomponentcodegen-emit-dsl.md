---
id: targets-makepad-components-codegen-makepadcomponentcodegen-emit-dsl
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 74
author: 
created: 
title: "MakepadComponentCodegen::emit_dsl"
tags: []
signature: "fn emit_dsl (& self , doc : & Document , reg : & crawrad_registry :: Registry , node : & Node , x : f32 , y : f32 , w : f32 , h : f32 ,) -> Result < String , Erro >"
---

Emits the LiveDesign Makepad DSL block.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::button::ButtonCodegen;
use crawrad_codegen::targets::makepad::components::codegen::MakepadComponentCodegen;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let codegen = ButtonCodegen;
let doc = Document::new("MainForm", "Form");
let reg = Registry::builtin();
let node = Node::new("btn", "Button");
let dsl = codegen.emit_dsl(&doc, &reg, &node, 0.0, 0.0, 100.0, 30.0).unwrap();
assert!(dsl.contains("btn = <Button>"));
```

# Errors

Returns an error if DSL generation fails.
