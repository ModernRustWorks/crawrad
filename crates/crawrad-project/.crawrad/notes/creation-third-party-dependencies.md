---
id: creation-third-party-dependencies
kind: doc
target_file: src/creation.rs
line_hint: 396
author: 
created: 
title: "third_party_dependencies"
tags: []
signature: "pub fn third_party_dependencies (doc : & crawrad_ir :: Document , destination : & Path) -> Option < String >"
---

Adds dependencies of third-party component crates to the project's Cargo.toml.

# Examples

```rust,no_run
// Does not modify Cargo.toml during doctests.
use std::path::Path;
use crawrad_ir::Document;
use crawrad_project::creation::third_party_dependencies;

let doc = Document::new("form", "Form");
let missing = third_party_dependencies(&doc, Path::new("."));
assert!(missing.is_none());
```
