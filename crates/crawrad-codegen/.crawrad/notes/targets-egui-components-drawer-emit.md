---
id: targets-egui-components-drawer-emit
kind: doc
target_file: src/targets/egui/components/drawer.rs
line_hint: 38
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits egui code generation for a Drawer container component.

A Drawer container (#500): side panel with header, optional close button,
child controls, and an optional dimmed scrim over the rest of the form.

# Examples

```rust
use crawrad_codegen::targets::egui::components::drawer::emit;
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("drawer", "Drawer");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "drawer", radios: &[], reg: &reg };
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "Drawer");
```

# Errors

Returns an error if property values or identifiers are invalid.

# Panics

Panics if the `Drawer` descriptor is not present in the registry.
