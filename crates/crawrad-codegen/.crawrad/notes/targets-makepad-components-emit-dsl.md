---
id: targets-makepad-components-emit-dsl
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 371
author: 
created: 
title: "emit_dsl"
tags: []
signature: "pub fn emit_dsl (doc : & Document , reg : & crawrad_registry :: Registry , node : & Node , x : f32 , y : f32 , w : f32 , h : f32 ,) -> Result < String , Erro >"
---

Emits Makepad DSL for widget instantiation.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::emit_dsl;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let reg = Registry::builtin();
let node = Node::new("btn", "Button");
let dsl = emit_dsl(&doc, &reg, &node, 0.0, 0.0, 100.0, 30.0).unwrap();
assert!(dsl.contains("btn = <Button>"));
```

# Errors

Returns an error if an identifier name is invalid or DSL generation fails.
