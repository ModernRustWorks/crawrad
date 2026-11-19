---
id: gerador-generate-with-base
kind: doc
target_file: src/lib.rs
line_hint: 179
author: 
created: 
title: "Gerador::generate_with_base"
tags: []
signature: "fn generate_with_base (& self , doc : & crawrad_ir :: Document , nome_do_crate : & str , reg : & crawrad_registry :: Registry , base : Option < & crawrad_ir :: Document > , is_base : bool ,) -> Result < Vec < Arquivo > , Erro >"
---

Generates a form that may inherit from `base`, or be a base of others.

`base` is the resolved base document when `doc` inherits; `is_base` says
some form of the project inherits from `doc`, so it reports what fired
for the derived form to dispatch (#851).

# Examples

```
use crawrad_codegen::Gerador;
use crawrad_codegen::targets::egui::GeradorRxp;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let reg = Registry::builtin();
let mut base = Document::new("form_base", "Form");
let mut ok = Node::new("btn_ok", "Button");
ok.events.insert("on_click".into(), "btn_ok_click".into());
base.add_child("form_base", ok).unwrap();
let files = GeradorRxp
    .generate_with_base(&base, "app", &reg, None, true)
    .unwrap();
assert!(files.iter().any(|f| f.conteudo.contains("show_reporting")));
```

# Errors

The same as [`Gerador::gerar`]: a document the target cannot generate.
