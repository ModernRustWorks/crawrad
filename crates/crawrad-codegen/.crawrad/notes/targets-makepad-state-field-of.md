---
id: targets-makepad-state-field-of
kind: doc
target_file: src/targets/makepad/state.rs
line_hint: 169
author: 
created: 
title: "field_of"
tags: []
signature: "pub fn field_of (class : & str , key : & str , default : & Value) -> Field"
---

The field in which this target stores `key` of the `class` class.

Without an entry in the valve, it follows the convention: the property name and the type implied
by the default value.

# Examples

```rust
use crawrad_codegen::targets::makepad::state::field_of;
use crawrad_ir::Value;

let f = field_of("Button", "text", &Value::Str(String::new()));
assert_eq!(f.ty, "String");
```
