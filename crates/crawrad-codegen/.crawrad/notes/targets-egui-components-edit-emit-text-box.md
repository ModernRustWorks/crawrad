---
id: targets-egui-components-edit-emit-text-box
kind: doc
target_file: src/targets/egui/components/edit.rs
line_hint: 27
author: 
created: 
title: "emit_text_box"
tags: []
signature: "pub fn emit_text_box (ctx : & ContextoEmissao , class : & 'static str , after_change : & str ,) -> Result < ResultadoComponente , Erro >"
---

Emits egui code generation for a single-line text box of the Edit family.

The body of a single-line text box of the `Edit` family, for `class`:
`after_change` goes in right after the typed text is held to its case,
before `OnChange` runs (a `MaskEdit` reshapes it there, #71).

# Examples

```rust
use crawrad_codegen::targets::egui::components::edit::emit_text_box;
use crawrad_codegen::targets::egui::components::ContextoEmissao;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("ed", "Edit");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "ed", radios: &[], reg: &reg };
let res = emit_text_box(&ctx, "Edit", "").unwrap();
assert_eq!(res.tipo_usado, "Edit");
```

# Errors

Returns an error if property values or identifiers are invalid.

# Panics

Panics if the text box `class` is not present in the registry.
