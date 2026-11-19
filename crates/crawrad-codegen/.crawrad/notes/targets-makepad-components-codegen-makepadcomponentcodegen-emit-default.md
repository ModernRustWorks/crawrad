---
id: targets-makepad-components-codegen-makepadcomponentcodegen-emit-default
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 34
author: 
created: 
title: "MakepadComponentCodegen::emit_default"
tags: []
signature: "fn emit_default (& self , node : & Node) -> String"
---

Emits the initial default state instantiation.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::button::ButtonCodegen;
use crawrad_codegen::targets::makepad::components::codegen::MakepadComponentCodegen;
use crawrad_ir::Node;

let codegen = ButtonCodegen;
let node = Node::new("btn", "Button");
let def = codegen.emit_default(&node);
assert!(def.contains("Button {"));
```
