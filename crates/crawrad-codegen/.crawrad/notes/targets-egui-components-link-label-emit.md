---
id: targets-egui-components-link-label-emit
kind: doc
target_file: src/targets/egui/components/link_label.rs
line_hint: 26
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits egui code generation for a LinkLabel component.

**A LinkLabel** (#500): the caption as egui's own link. A click opens
`url` in the system's browser (web and mail only, `SHELL` emitted once),
then runs `OnClick`.

# Examples

```rust
use crawrad_codegen::targets::egui::components::link_label::emit;
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("link", "LinkLabel");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "link", radios: &[], reg: &reg };
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "LinkLabel");
```

# Errors

Returns an error if property values or identifiers are invalid.

# Panics

Panics if the `LinkLabel` descriptor is not present in the registry.
