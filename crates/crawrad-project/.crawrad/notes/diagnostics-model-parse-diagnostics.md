---
id: diagnostics-model-parse-diagnostics
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 100
author: 
created: 
title: "parse_diagnostics"
tags: []
signature: "pub fn parse_diagnostics (output : & str) -> Vec < Diagnostic >"
---

Parses standard output of `cargo build --message-format=json`.

# Examples

```rust
use crawrad_project::diagnostics_model::parse_diagnostics;

let diags = parse_diagnostics("");
assert!(diags.is_empty());
```
