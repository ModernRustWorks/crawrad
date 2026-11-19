---
id: targets-egui-components-spin-edit-emit
kind: doc
target_file: src/targets/egui/components/spin_edit.rs
line_hint: 23
author: 
created: 
title: "emit"
tags: []
signature: "pub fn emit (ctx : & ContextoEmissao) -> Result < ResultadoComponente , Erro >"
---

Emits egui code generation for a SpinEdit component.

**A SpinEdit** (#500): the number in a field and two buttons at its
right, a step up and a step down. `MinValue`/`MaxValue` bound it unless
both are 0; `EditorEnabled` off leaves the field read-only and the buttons
working. `OnChange` runs when the value moved.

# Examples

```rust
use crawrad_codegen::targets::egui::components::spin_edit::emit;
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("se", "SpinEdit");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "se", radios: &[], reg: &reg };
let res = emit(&ctx).unwrap();
assert_eq!(res.tipo_usado, "SpinEdit");
```

# Errors

Returns an error if property values or identifiers are invalid.

# Panics

Panics if the `SpinEdit` descriptor is not present in the registry.
