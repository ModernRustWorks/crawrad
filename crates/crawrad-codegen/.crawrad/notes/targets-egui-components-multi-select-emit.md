---
id: targets-egui-components-multi-select-emit
kind: doc
target_file: src/targets/egui/components/multi_select.rs
line_hint: 24
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

A MultiSelect component: a drop-down field with checkboxes.

**A MultiSelect component** (#500): a drop-down field with checkboxes.

# Examples

```rust
use crawrad_codegen::targets::egui::components::multi_select::emit;
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("ms", "MultiSelect");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "ms", radios: &[], reg: &reg };
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "MultiSelect");
```

# Errors

Returns an error if property values or identifiers are invalid.

# Panics

Panics if the `MultiSelect` descriptor is not present in the registry.
