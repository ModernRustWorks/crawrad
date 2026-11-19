---
id: targets-egui-components-pagination-emit
kind: doc
target_file: src/targets/egui/components/pagination.rs
line_hint: 23
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

A Pagination component: a row of page buttons.

**A Pagination component** (#500): a row of page buttons.

# Examples

```rust
use crawrad_codegen::targets::egui::components::pagination::emit;
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("pg", "Pagination");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "pg", radios: &[], reg: &reg };
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "Pagination");
```

# Errors

Returns an error if property values or identifiers are invalid.

# Panics

Panics if the `Pagination` descriptor is not present in the registry.
