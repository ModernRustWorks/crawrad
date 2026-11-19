---
id: render-cx
kind: doc
target_file: src/render.rs
line_hint: 12
author: 
created: 
title: "Cx"
tags: []
signature: "pub struct Cx < 'a >"
---

Context holding the value source and optional type index.

What a value is formatted from: the debugger, and — when the program's
DWARF was read — its type layouts.

# Examples

```rust,ignore
// private module: doctests cannot reach items in non-public modules
use crawrad_dap_rust::render::Cx;
```
