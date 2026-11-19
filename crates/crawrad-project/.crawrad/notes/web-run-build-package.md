---
id: web-run-build-package
kind: doc
target_file: src/web_run.rs
line_hint: 77
author: 
created: 
title: "build_package"
tags: []
signature: "pub fn build_package (root : & Path , package : & str) -> Result < PathBuf , String >"
---

Builds the WebAssembly release package and bundles web runtime assets.

# Errors

Returns the last lines of cargo's report when the wasm build or `cargo metadata`
fails, or a message when a file of the package cannot be copied or written.

# Examples

```rust
use std::path::Path;
use crawrad_project::web_run::build_package;

let res = build_package(Path::new("missing"), "pkg");
assert!(res.is_err());
```
