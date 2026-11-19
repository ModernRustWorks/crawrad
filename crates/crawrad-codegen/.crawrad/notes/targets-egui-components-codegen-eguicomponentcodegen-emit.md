---
id: targets-egui-components-codegen-eguicomponentcodegen-emit
kind: doc
target_file: src/targets/egui/components/codegen.rs
line_hint: 12
author: 
created: 
title: "EguiComponentCodegen::emit"
tags: []
signature: "fn emit (& self , ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits the egui code generation outcome for this component.

# Examples

```rust
use crawrad_codegen::targets::egui::components::button::ButtonCodegen;
use crawrad_codegen::targets::egui::components::{ContextoEmissao, EguiComponentCodegen};
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "btn", radios: &[], reg: &reg };
let codegen = ButtonCodegen;
let res = codegen.emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "Button");
```

# Errors

Returns an error if component code generation fails.
