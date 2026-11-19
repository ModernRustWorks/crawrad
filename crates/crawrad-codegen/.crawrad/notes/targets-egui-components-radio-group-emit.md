---
id: targets-egui-components-radio-group-emit
kind: doc
target_file: src/targets/egui/components/radio_group.rs
line_hint: 24
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits egui code generation for a RadioGroup component.

**A RadioGroup** (#500): the group box frame with its caption, and one
radio per item laid out in `columns` columns, filling each column top
down as the classic RAD IDE does. A click sets `item_index`; `OnClick`
runs on every click, `OnChange` only when the choice moved.

# Examples

```rust
use crawrad_codegen::targets::egui::components::radio_group::emit;
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("rg", "RadioGroup");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "rg", radios: &[], reg: &reg };
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "RadioGroup");
```

# Errors

Returns an error if property values or identifiers are invalid.

# Panics

Panics if the `RadioGroup` descriptor is not present in the registry.
