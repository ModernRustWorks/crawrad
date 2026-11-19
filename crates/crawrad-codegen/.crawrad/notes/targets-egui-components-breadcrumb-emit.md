---
id: targets-egui-components-breadcrumb-emit
kind: doc
target_file: src/targets/egui/components/breadcrumb.rs
line_hint: 24
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

**A Breadcrumb component** (#500): a navigation trail on one line.

# Examples

```rust
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_codegen::targets::egui::components::breadcrumb::emit;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("bc1", "Breadcrumb");
let reg = Registry::builtin();
let ctx = ContextoEmissao {
    doc: &doc,
    node: &node,
    nome: "bc1",
    radios: &[],
    reg: &reg,
};
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "Breadcrumb");
```

# Errors

Returns an error if any handler identifier is invalid.

# Panics

Panics if the Breadcrumb descriptor is missing from the registry.
