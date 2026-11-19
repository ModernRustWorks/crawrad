---
id: targets-egui-state-value-to-literal
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 970
author: 
created: 
title: "value_to_literal"
tags: []
signature: "pub fn value_to_literal (field_ty : & str , val : & Value) -> String"
---

Converts a property value to a literal for the given field type.

# Examples

```rust
use crawrad_codegen::targets::egui::state::value_to_literal;
use crawrad_ir::Value;

let lit = value_to_literal("bool", &Value::Bool(true));
assert_eq!(lit, "true");
```
