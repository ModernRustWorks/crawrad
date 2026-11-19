---
id: creation-missing-dependencies
kind: doc
target_file: src/creation.rs
line_hint: 396
author: 
created: 
title: "missing_dependencies"
tags: []
signature: "pub fn missing_dependencies (manifest: & str, deps: Vec < crawrad_codegen :: Dependencia >) -> Vec < crawrad_codegen :: Dependencia >"
---

Filters dependencies down to those not yet listed under `[dependencies]`.

Reads lines within the `[dependencies]` section of a `Cargo.toml` manifest,
skipping any dependency whose key is already present.

# Examples

```rust
use crawrad_codegen::Dependencia;
use crawrad_project::creation::missing_dependencies;

let manifest = "[dependencies]\nmakepad-widgets = \"1.0\"\n";
let deps = vec![
    Dependencia::registro("makepad-widgets", "1.0"),
    Dependencia::registro("crawrad-runtime", "0.1"),
];
let missing = missing_dependencies(manifest, deps);
assert_eq!(missing.len(), 1);
assert_eq!(missing[0].nome(), "crawrad-runtime");
```
