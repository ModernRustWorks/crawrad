---
id: dependencies-dependencias-do-projeto
kind: doc
target_file: src/dependencies.rs
line_hint: 120
author: 
created: 
title: "dependencias_do_projeto"
tags: []
signature: "pub fn dependencias_do_projeto (alvo : AlvoGerador , com_formulario : bool , usa_binding : bool ,) -> Vec < Dependencia >"
---

Lists the project dependencies for the specified target.

Lists all dependencies the project needs to have according to the target,
whether it has a form, and whether it uses data binding.

# Examples

```rust
use crawrad_codegen::{AlvoGerador, dependencias_do_projeto};

let deps = dependencias_do_projeto(AlvoGerador::Makepad, false, false);
assert!(deps.is_empty());
```
