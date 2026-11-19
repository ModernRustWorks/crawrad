---
id: makepadruntime-form-rect
kind: doc
target_file: src/lib.rs
line_hint: 365
author: 
created: 
title: "MakepadRuntime::form_rect"
tags: []
signature: "pub fn form_rect (& self) -> Rect"
---

The form's rectangle, in design units.

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_runtime_makepad::MakepadRuntime;

let rt = MakepadRuntime::new();
assert_eq!(rt.form_rect(), Rect::new(0.0, 0.0, 640.0, 480.0));
```
