---
id: targets-data-binding-dataset-action-enabled-expr
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 104
author: 
created: 
title: "dataset_action_enabled_expr"
tags: []
signature: "pub fn dataset_action_enabled_expr (operation : & str , adapter : & str) -> String"
---

Returns the boolean expression for whether a dataset action is enabled.

Returns the boolean expression determining whether a dataset action is
enabled (ADR 0041, #788).

Pure function used by both Makepad and egui targets.

# Examples

```rust
use crawrad_codegen::targets::data_binding::dataset_action_enabled_expr;

let expr = dataset_action_enabled_expr("First", "adapter");
assert!(expr.contains("adapter.state().get().is_open()"));
```
