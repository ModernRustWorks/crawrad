---
id: targets-data-binding-makepad-model-to-widget
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 427
author: 
created: 
title: "makepad_model_to_widget"
tags: []
signature: "pub fn makepad_model_to_widget (controls : & [BoundControl]) -> String"
---

Makepad widget sync lines for scalar bound controls.

# Examples

```rust
use crawrad_codegen::targets::data_binding::makepad_model_to_widget;

let code = makepad_model_to_widget(&[]);
assert!(code.is_empty());
```
