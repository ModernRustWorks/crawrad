---
id: targets-egui-components-chips-emit
kind: doc
target_file: src/targets/egui/components/chips.rs
line_hint: 24
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits egui code for a Chips component.

**A Chips component** (#500): a group of rounded chips laid out left to right,
wrapping to the next line when the row is full.

# Examples

```rust
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_codegen::targets::egui::components::chips::emit;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("chips1", "Chips");
let reg = Registry::builtin();
let ctx = ContextoEmissao {
    doc: &doc,
    node: &node,
    nome: "chips1",
    radios: &[],
    reg: &reg,
};
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "Chips");
```

# Errors

Returns an error if any handler identifier is invalid.

# Panics

Panics if the Chips descriptor is missing from the registry.
