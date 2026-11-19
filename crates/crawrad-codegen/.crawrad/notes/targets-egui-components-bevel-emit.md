---
id: targets-egui-components-bevel-emit
kind: doc
target_file: src/targets/egui/components/bevel.rs
line_hint: 22
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits egui code for a Bevel component.

**A Bevel** (#500): the lines the designer draws, from the same
`paint_bevel` (the `bevel` section of the shared drawing, which the form
emits once).

# Examples

```rust
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_codegen::targets::egui::components::bevel::emit;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("bevel1", "Bevel");
let reg = Registry::builtin();
let ctx = ContextoEmissao {
    doc: &doc,
    node: &node,
    nome: "bevel1",
    radios: &[],
    reg: &reg,
};
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "Bevel");
```

# Errors

Returns an error if emitting the component state fails.

# Panics

Panics if the Bevel descriptor is missing from the registry.
