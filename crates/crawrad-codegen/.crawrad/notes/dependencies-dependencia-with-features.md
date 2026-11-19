---
id: dependencies-dependencia-with-features
kind: doc
target_file: src/dependencies.rs
line_hint: 47
author: 
created: 
title: "Dependencia::with_features"
tags: []
signature: "pub fn with_features (mut self , wanted : & [& str]) -> Self"
---

Returns the dependency with additional features enabled.

The same dependency with `features` turned on (a path dependency only;
a registry one is returned unchanged).

# Examples

```rust
use std::path::PathBuf;
use crawrad_codegen::Dependencia;

let dep = Dependencia::caminho("my-crate", PathBuf::from("../my-crate"))
    .with_features(&["feat1"]);
assert_eq!(dep.args_cargo_add(), vec!["--path", "../my-crate", "--features", "feat1"]);
```
