---
id: targets-egui-components-corpo-fecha
kind: doc
target_file: src/targets/egui/components/mod.rs
line_hint: 208
author: 
created: 
title: "corpo_fecha"
tags: []
signature: "pub fn corpo_fecha (ctx : & ContextoEmissao) -> Result < String , Erro >"
---

What closes the component block, AFTER its children.

# Examples

```rust
use crawrad_codegen::targets::egui::components::{corpo_fecha, ContextoEmissao};
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let reg = Registry::builtin();
let ctx = ContextoEmissao { doc: &doc, node: &node, nome: "btn", radios: &[], reg: &reg };
let fecho = corpo_fecha(&ctx).unwrap();
assert!(fecho.is_empty());
```

# Errors

Returns an error if closing the component block fails.
