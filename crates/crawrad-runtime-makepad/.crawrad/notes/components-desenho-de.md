---
id: components-desenho-de
kind: doc
target_file: src/components/mod.rs
line_hint: 157
author: 
created: 
title: "desenho_de"
tags: []
signature: "pub fn desenho_de (classe : & str) -> & 'static dyn ComponentDrawing"
---

The drawing of a class — there is always one.

Anything not in the catalog falls back to [`generic::Desenho`], which paints the
dashed rectangle with the class name. Therefore, the caller has no
`Option` to handle, and there is no second place to decide what to do
with an unknown class.

# Examples

```rust
use crawrad_runtime_makepad::components::desenho_de;

let drawing = desenho_de("Button");
assert_eq!(drawing.class_name(), "Button");
```
