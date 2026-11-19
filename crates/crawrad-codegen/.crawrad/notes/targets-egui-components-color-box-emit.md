---
id: targets-egui-components-color-box-emit
kind: doc
target_file: src/targets/egui/components/color_box.rs
line_hint: 23
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits egui code generation for a ColorBox component.

**A ColorBox** (#500): the swatch of the chosen colour and a drop-down of
the standard colours, each row with its swatch. `OnChange` runs when the
choice moved; a colour that is not one of the list shows as "Custom".

# Examples

```rust
use crawrad_codegen::targets::egui::components::color_box::emit;
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("cb", "ColorBox");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "cb", radios: &[], reg: &reg };
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "ColorBox");
```

# Errors

Returns an error if property values or identifiers are invalid.

# Panics

Panics if the `ColorBox` descriptor is not present in the registry.
