---
id: designer-ruler-steps
kind: doc
target_file: src/designer/ruler.rs
line_hint: 32
author: 
created: 
title: "STEPS"
tags: []
signature: "pub const STEPS : [f32 ; 8] = [5.0 , 10.0 , 25.0 , 50.0 , 100.0 , 250.0 , 500.0 , 1000.0]"
---

The steps the ruler can choose from, in design units.

They are any ruler's steps: multiples of 5 and 10, because the arithmetic
the user does by eye is always mental. Mirrors the RXP's `PASSOS`
(`crawrad-runtime-egui/src/lib.rs:3311`).

# Examples

```rust,ignore
// Not reachable from outside the crate: shown, not run.
use crate::designer::ruler::STEPS;

assert_eq!(STEPS[0], 5.0);
```
