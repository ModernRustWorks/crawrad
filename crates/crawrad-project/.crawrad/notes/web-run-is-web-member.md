---
id: web-run-is-web-member
kind: doc
target_file: src/web_run.rs
line_hint: 33
author: 
created: 
title: "is_web_member"
tags: []
signature: "pub fn is_web_member (member : & Path) -> bool"
---

Whether the member folder's own `Cargo.toml` marks it as a browser app.

Reads `[package.metadata.crawrad] platform = "web"` straight from the file,
without `cargo metadata`, so the Toolbox and the Designer can ask while they draw.

# Examples

```rust
use std::path::Path;
use crawrad_project::web_run::is_web_member;

assert!(!is_web_member(Path::new("missing")));
```
