---
id: targets-egui-components-badge-emit
kind: doc
target_file: src/targets/egui/components/badge.rs
line_hint: 24
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

**A Badge** (#500): a pill label or notification badge drawn via `paint_badge`.

An optional click handler triggers `OnClick`.

# Examples

```rust
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_codegen::targets::egui::components::badge::emit;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("badge1", "Badge");
let reg = Registry::builtin();
let ctx = ContextoEmissao {
    doc: &doc,
    node: &node,
    nome: "badge1",
    radios: &[],
    reg: &reg,
};
let res = emit(&ctx).unwrap();
assert!(!res.corpo.is_empty());
```

# Errors

Returns an error if the click handler identifier is invalid.

# Panics

Panics if the Badge descriptor is missing from the registry.
