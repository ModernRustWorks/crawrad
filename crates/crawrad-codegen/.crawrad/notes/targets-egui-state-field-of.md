---
id: targets-egui-state-field-of
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 420
author: 
created: 
title: "field_of"
tags: []
signature: "pub fn field_of (class : & str , key : & str , default : & Value) -> Field"
---

Returns the field definition for `class` and `key`.

# Examples

```rust
use crawrad_codegen::targets::egui::state::field_of;
use crawrad_ir::Value;

let f = field_of("Button", "text", &Value::Str("".into()));
assert_eq!(f.name, "");
```
