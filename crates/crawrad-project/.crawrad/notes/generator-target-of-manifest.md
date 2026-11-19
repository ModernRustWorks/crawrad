---
id: generator-target-of-manifest
kind: doc
target_file: src/generator.rs
line_hint: 121
author: 
created: 
title: "target_of_manifest"
tags: []
signature: "pub fn target_of_manifest (manifest : & str) -> Option < crawrad_codegen :: AlvoGerador >"
---

Reads the UI framework a project is built with from its `Cargo.toml`.

The UI framework a project is built with, read from its `Cargo.toml`
(#565): the dependency New Project wrote is what the code compiles
against, so it is the one record — no second copy to drift from it.
The CrawRAD runtime the generated code uses speaks first; failing it, the
one framework named. `None` when it is neither, or both (a project may
carry the other framework for a tool of its own).

# Examples

```rust
use crawrad_codegen::AlvoGerador;
use crawrad_project::generator::target_of_manifest;

let manifest = r#"
[dependencies]
makepad-widgets = "1.0"
"#;
assert_eq!(target_of_manifest(manifest), Some(AlvoGerador::Makepad));
```
