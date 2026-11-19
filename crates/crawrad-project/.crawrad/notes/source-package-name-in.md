---
id: source-package-name-in
kind: doc
target_file: src/source.rs
line_hint: 68
author: 
created: 
title: "package_name_in"
tags: []
signature: "pub fn package_name_in (manifest : & str) -> Option < String >"
---

The `name` of the `[package]` section, if the manifest declares one.

Deliberately not a TOML parser: the shape is `name = "x"` inside
`[package]`, and the egui shell reads it the same way for the same reason —
a dependency for one key is a dependency too many.

# Examples

```rust
use crawrad_project::source::package_name_in;

let toml = "[package]\nname = \"my_pkg\"\n";
assert_eq!(package_name_in(toml), Some("my_pkg".to_string()));
```
