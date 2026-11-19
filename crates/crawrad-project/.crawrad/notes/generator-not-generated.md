---
id: generator-not-generated
kind: doc
target_file: src/generator.rs
line_hint: 155
author: 
created: 
title: "NOT_GENERATED"
tags: []
signature: "pub const NOT_GENERATED : & str = \"rust-analyzer (project not generated — Ctrl+S)\""
---

The status bar's word for a project rust-analyzer cannot analyse yet.

# Examples

```rust
use crawrad_project::generator::NOT_GENERATED;

assert!(NOT_GENERATED.contains("rust-analyzer"));
```
