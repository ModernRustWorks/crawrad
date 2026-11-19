---
id: designer-ruler-ruler-thickness
kind: doc
target_file: src/designer/ruler.rs
line_hint: 26
author: 
created: 
title: "RULER_THICKNESS"
tags: []
signature: "pub const RULER_THICKNESS : f32 = 18.0"
---

The ruler thickness, **in screen points**.

It is not a design unit, and it is the only canvas measure that is not. The
reason is the one the RXP already wrote
(`crawrad-runtime-egui/src/lib.rs:94-99`): *"the ruler is not part of the
drawing, it measures it. A ruler that shrank with the zoom would take the
number written on it along, and the number is why it exists."*

# Examples

```rust
use crawrad_runtime_makepad::designer::RULER_THICKNESS;

assert_eq!(RULER_THICKNESS, 18.0);
```
