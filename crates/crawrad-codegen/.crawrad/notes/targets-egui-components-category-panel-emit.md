---
id: targets-egui-components-category-panel-emit
kind: doc
target_file: src/targets/egui/components/category_panel.rs
line_hint: 39
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits egui code for a CategoryPanel component.

**A CategoryPanel** (#500): the header from the shared drawing
(`paint_category`, which the form emits once); a click on it folds the
panel and runs `OnCollapsed` / `OnExpanded`.

# Examples

```rust
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_codegen::targets::egui::components::category_panel::emit;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("cp1", "CategoryPanel");
let reg = Registry::builtin();
let ctx = ContextoEmissao {
    doc: &doc,
    node: &node,
    nome: "cp1",
    radios: &[],
    reg: &reg,
};
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "CategoryPanel");
```

# Errors

Returns an error if any event handler identifier is invalid.

# Panics

Panics if the CategoryPanel descriptor is missing from the registry.
