---
id: targets-makepad-components-util-format-field
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 57
author: 
created: 
title: "format_field"
tags: []
signature: "pub fn format_field (name : & str , ty : & str) -> String"
---

Formats a Rust struct field line.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::format_field;

assert_eq!(format_field("name", "String"), "    pub name: String,");
```
