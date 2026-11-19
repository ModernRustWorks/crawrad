---
id: targets-egui-components-toggle-emit
kind: doc
target_file: src/targets/egui/components/toggle.rs
line_hint: 23
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits egui code generation for a Toggle switch component.

**A Toggle** (#500): a switch — a track with a knob that slides to the
right when on — and the caption beside it. A click anywhere on it flips
`checked`; `OnClick`, then `OnChange`, as the CheckBox orders them.

# Examples

```rust
use crawrad_codegen::targets::egui::components::toggle::emit;
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("tg", "Toggle");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "tg", radios: &[], reg: &reg };
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "Toggle");
```

# Errors

Returns an error if property values or identifiers are invalid.

# Panics

Panics if the `Toggle` descriptor is not present in the registry.
