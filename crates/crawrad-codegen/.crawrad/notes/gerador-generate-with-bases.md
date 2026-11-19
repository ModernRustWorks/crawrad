---
id: gerador-generate-with-bases
kind: doc
target_file: src/lib.rs
line_hint: 192
author:
created:
title: "generate_with_bases"
tags: []
signature: "fn generate_with_bases (& self , doc : & crawrad_ir :: Document , nome_do_crate : & str , reg : & crawrad_registry :: Registry , bases : & [crawrad_ir :: Document] , is_base : bool ,) -> Result < Vec < Arquivo > , Erro >"
---

Generates a form that inherits a chain of bases, nearest first.

`bases[0]` is the form's own base, `bases[1]` that base's base, and so on:
a form three levels down reaches every event above it.

# Examples

```
use crawrad_codegen::Gerador;
use crawrad_codegen::targets::egui::GeradorRxp;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let reg = Registry::builtin();
let mut a = Document::new("form_a", "Form");
let mut ok = Node::new("btn_ok", "Button");
ok.events.insert("on_click".into(), "btn_ok_click".into());
a.add_child("form_a", ok).unwrap();
let mut b = Document::new("form_b", "Form");
b.set_inherits(Some("form_a"));
let files = GeradorRxp
    .generate_with_bases(&b, "app", &reg, &[a], true)
    .unwrap();
// B reports A's events up, wrapped, for a form further down
assert!(files.iter().any(|f| f.conteudo.contains("Base(FormAFired)")));
```

# Errors

The same as [`Gerador::gerar`]: a document the target cannot generate.
