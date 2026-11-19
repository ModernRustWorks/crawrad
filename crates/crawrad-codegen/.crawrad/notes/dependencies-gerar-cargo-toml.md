---
id: dependencies-gerar-cargo-toml
kind: doc
target_file: src/dependencies.rs
line_hint: 246
author: 
created: 
title: "gerar_cargo_toml"
tags: []
signature: "pub fn gerar_cargo_toml (nome : & str , alvo : AlvoGerador , com_formulario : bool , usa_binding : bool , extras : & [Dependencia] ,) -> String"
---

Generates the contents of a `Cargo.toml` manifest for the project.

Generates the complete contents of a basic `Cargo.toml` with dependencies
calculated from the single list `dependencias_do_projeto` and optional extras.

# Examples

```rust
use crawrad_codegen::{AlvoGerador, dependencies::gerar_cargo_toml};

let toml = gerar_cargo_toml("meu_app", AlvoGerador::Makepad, false, false, &[]);
assert!(toml.contains("meu_app"));
```
