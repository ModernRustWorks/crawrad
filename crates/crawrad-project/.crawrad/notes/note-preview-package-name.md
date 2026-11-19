---
id: note-preview-package-name
kind: doc
target_file: src/note_preview.rs
line_hint: 103
author: 
created: 
title: "package_name"
tags: []
signature: "pub fn package_name (cargo_toml : & str) -> Option < String >"
---

The `[package] name` of a `Cargo.toml`.

# Examples

```rust
use crawrad_project::note_preview::package_name;

let cargo = "[package]\nname = \"my_pkg\"\n";
assert_eq!(package_name(cargo), Some("my_pkg".to_string()));
```
