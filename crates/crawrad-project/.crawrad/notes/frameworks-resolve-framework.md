---
id: frameworks-resolve-framework
kind: doc
target_file: src/frameworks.rs
line_hint: 72
author: 
created: 
title: "resolve_framework"
tags: []
signature: "pub fn resolve_framework (name : Option < & str >) -> Result < AlvoGerador , String >"
---

Resolves an optional framework name (case-insensitive) to a generator target.

Defaults to Makepad when omitted or empty.

# Errors

Returns an error if the framework is unknown or only planned.

# Examples

```rust
use crawrad_codegen::AlvoGerador;
use crawrad_project::frameworks::resolve_framework;

assert_eq!(resolve_framework(None).unwrap(), AlvoGerador::Makepad);
assert_eq!(resolve_framework(Some("egui")).unwrap(), AlvoGerador::Egui);
```
