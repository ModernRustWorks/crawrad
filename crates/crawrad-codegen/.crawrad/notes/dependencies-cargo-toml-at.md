---
id: dependencies-cargo-toml-at
kind: doc
target_file: src/dependencies.rs
line_hint: 317
author: 
created: 
title: "cargo_toml_at"
tags: []
signature: "pub fn cargo_toml_at (manifest_dir : & Path , nome : & str , alvo : AlvoGerador , com_formulario : bool , usa_binding : bool , extras : & [Dependencia] ,) -> String"
---

Cargo.toml with path deps relative to the manifest folder.

# Examples

```rust
use crawrad_codegen::{AlvoGerador, dependencies::cargo_toml_at};

let toml = cargo_toml_at(std::path::Path::new("."), "app", AlvoGerador::Makepad, true, false, &[]);
assert!(toml.contains("name = \"app\""));
```
