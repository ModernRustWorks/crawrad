---
id: targets-egui-components-check-list-box-emit
kind: doc
target_file: src/targets/egui/components/check_list_box.rs
line_hint: 28
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

**A CheckListBox component**: a vertical scrollable list with checkboxes.

# Examples

```rust
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_codegen::targets::egui::components::check_list_box::emit;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("clb1", "CheckListBox");
let reg = Registry::builtin();
let ctx = ContextoEmissao {
    doc: &doc,
    node: &node,
    nome: "clb1",
    radios: &[],
    reg: &reg,
};
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "CheckListBox");
```

# Errors

Returns an error if any event handler identifier is invalid.

# Panics

Panics if the CheckListBox descriptor is missing from the registry.
