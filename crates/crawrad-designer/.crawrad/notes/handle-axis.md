---
id: handle-axis
kind: doc
target_file: src/lib.rs
line_hint: 85
author: 
created: 
title: "Handle::axis"
tags: []
signature: "pub fn axis (self) -> Axis"
---

The arrow that this handle requests, by the axis along which it stretches.

The designer knows the geometry; the adapter knows how to draw arrows.
So what comes out here is the **axis name**, not a toolkit cursor:
`Nwse`, `Nesw`, `We`, `Ns`. A new adapter maps four names.

# Examples

```rust
use crawrad_designer::{Axis, Handle};

assert_eq!(Handle::NW.axis(), Axis::Nwse);
```
