---
id: targets-egui-state-field
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 10
author: 
created: 
title: "Field"
tags: []
signature: "pub struct Field"
---

How the egui target holds a property when not using standard conventions.

# Examples

```rust
use crawrad_codegen::targets::egui::state::Field;

let f = Field { name: "selected", ty: "Option<usize>" };
assert_eq!(f.name, "selected");
```
