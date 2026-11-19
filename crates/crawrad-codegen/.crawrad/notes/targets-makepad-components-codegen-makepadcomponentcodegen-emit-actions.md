---
id: targets-makepad-components-codegen-makepadcomponentcodegen-emit-actions
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 45
author: 
created: 
title: "MakepadComponentCodegen::emit_actions"
tags: []
signature: "fn emit_actions (& self , doc : & Document , node : & Node , field : & str , emitted : & [& Node] ,) -> Result < String , Erro >"
---

Emits the `handle_actions` event dispatch block.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::button::ButtonCodegen;
use crawrad_codegen::targets::makepad::components::codegen::MakepadComponentCodegen;
use crawrad_ir::{Document, Node};

let codegen = ButtonCodegen;
let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let acts = codegen.emit_actions(&doc, &node, "btn", &[]).unwrap();
assert!(acts.is_empty());
```

# Errors

Returns an error if an identifier name is invalid.
