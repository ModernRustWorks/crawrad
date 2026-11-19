---
id: gerador-gerar
kind: doc
target_file: src/lib.rs
line_hint: 169
author: 
created: 
title: "Gerador::gerar"
tags: []
signature: "fn gerar (& self , doc : & crawrad_ir :: Document , nome_do_crate : & str , reg : & crawrad_registry :: Registry ,) -> Result < Vec < Arquivo > , Erro >"
---

Generates the project files in memory, without touching disk.

The project files, without touching disk. Separating deciding from writing
is what makes generation testable without a file system.

# Examples

```rust
use crawrad_codegen::{Gerador, GeradorRxp};
use crawrad_ir::Document;
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let reg = Registry::builtin();
let arquivos = GeradorRxp.gerar(&doc, "meu_app", &reg).unwrap();
assert!(!arquivos.is_empty());
```

# Errors

Returns an error if the document has no root or contains invalid identifiers.

# `reg` is the CATALOG, and it has been a parameter since 2026-09-18

Before that, the signature was `(doc, nome_do_crate)`, and the generator
constructed `Registry::builtin()` on its own in seven places. The
consequence was tracked as a known gap: **a component registered
via `register_external` appeared in the palette, in the Object Inspector, and in
`.crawform`, and disappeared during code generation.**

It was the last privileged backdoor of
ADR 0035, and the
one keeping `Connection` from turning into code in the application.

Receiving it as a parameter costs one argument in each call. The
alternative — a process-current catalog — would cost a word and
**global state**: registration order and test isolation would start to
matter, and in a crate whose reason to exist is being testable without a UI, that
is too costly.
