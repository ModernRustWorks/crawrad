---
id: targets-egui-state-field-doc
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 443
author: 
created: 
title: "field_doc"
tags: []
signature: "pub fn field_doc (class : & str , field : & str) -> Option < & 'static str >"
---

Returns the doc comment for a field, if any.

# Examples

```rust
use crawrad_codegen::targets::egui::state::field_doc;

assert!(field_doc("Button", "cursor").is_some());
```
