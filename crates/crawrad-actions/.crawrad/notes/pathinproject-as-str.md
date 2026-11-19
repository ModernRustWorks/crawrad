---
id: pathinproject-as-str
kind: doc
target_file: src/lib.rs
line_hint: 313
author: 
created: 
title: "PathInProject::as_str"
tags: []
signature: "pub fn as_str (& self) -> & str"
---

The already-validated path, relative to the generated project root.

# Examples

```rust
use crawrad_actions::PathInProject;

let p = PathInProject::new("Cargo.toml").unwrap();
assert_eq!(p.as_str(), "Cargo.toml");
```
