---
id: workspace-package-name-from-manifest
kind: doc
target_file: src/workspace.rs
line_hint: 1826
author: 
created: 
title: "package_name_from_manifest"
tags: []
signature: "pub fn package_name_from_manifest (cargo_toml : & str) -> Option < String >"
---

The package name declared in a `Cargo.toml`.

ADR 0017's answer to *"what is the application name?"*: `[package] name`,
and not a separate field alongside it. A three-line parser instead of `toml`
crate, because the query is singular and the answer is always in the same shape —
and a full parser dependency to read one key is a dependency paid on every compilation.

Section matters: `name` also exists in `[[bin]]`, `[lib]` and long-form `[dependencies]`.
Only the `name` **inside `[package]`** answers.

# Examples

```rust
use crawrad_project::workspace::package_name_from_manifest;

let toml = "[package]\nname = \"my_pkg\"\n";
assert_eq!(package_name_from_manifest(toml), Some("my_pkg".to_string()));
```
