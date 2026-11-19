---
id: inspector-model-binding-mode-name
kind: doc
target_file: src/inspector_model.rs
line_hint: 165
author: 
created: 
title: "binding_mode_name"
tags: []
signature: "pub fn binding_mode_name (mode : crawrad_ir :: BindingMode) -> & 'static str"
---

Display name of a [`crawrad_ir::BindingMode`] for the Mode dropdown.

Canonical display name of a [`crawrad_ir::BindingMode`] for the Inspector's Mode dropdown.

# Examples

```rust
use crawrad_actions::inspector_model::binding_mode_name;
use crawrad_ir::BindingMode;

assert_eq!(binding_mode_name(BindingMode::OneWay), "OneWay");
```
