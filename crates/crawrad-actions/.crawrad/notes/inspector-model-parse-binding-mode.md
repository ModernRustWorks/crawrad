---
id: inspector-model-parse-binding-mode
kind: doc
target_file: src/inspector_model.rs
line_hint: 155
author: 
created: 
title: "parse_binding_mode"
tags: []
signature: "pub fn parse_binding_mode (s : & str) -> crawrad_ir :: BindingMode"
---

Parse a data binding mode string into a [`crawrad_ir::BindingMode`].

# Examples

```rust
use crawrad_actions::inspector_model::parse_binding_mode;
use crawrad_ir::BindingMode;

assert_eq!(parse_binding_mode("oneway"), BindingMode::OneWay);
```
