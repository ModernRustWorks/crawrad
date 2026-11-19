---
id: targets-egui-components-card-emit
kind: doc
target_file: src/targets/egui/components/card.rs
line_hint: 38
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits egui code for a Card container component.

A Card container (#500): rounded surface with elevation shadow, optional
header (title and subtitle). Children are placed relative to the card's rect.

# Examples

```rust
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_codegen::targets::egui::components::card::emit;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("card1", "Card");
let reg = Registry::builtin();
let ctx = ContextoEmissao {
    doc: &doc,
    node: &node,
    nome: "card1",
    radios: &[],
    reg: &reg,
};
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "Card");
```

# Errors

Returns an error if the click handler identifier is invalid.

# Panics

Panics if the Card descriptor is missing from the registry.
