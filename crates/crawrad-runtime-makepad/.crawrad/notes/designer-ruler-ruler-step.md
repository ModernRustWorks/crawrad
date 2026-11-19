---
id: designer-ruler-ruler-step
kind: doc
target_file: src/designer/ruler.rs
line_hint: 35
author: 
created: 
title: "ruler_step"
tags: []
signature: "pub fn ruler_step (scale : f32) -> f32"
---

The interval between two ruler NUMBERS, in design units.

It **depends on the zoom**, and it has to: a tick every 50 units is
comfortable at 100% and becomes a wall at 25%, where those same 50 units
fall within 12 screen points. The ruler picks the finest step that still
leaves [`RULER_LABEL_GAP`] points between two numbers.

This is the port of the RXP's `passo_da_regua`
(`crawrad-runtime-egui/src/lib.rs:3310`), with the same numbers — and the test
`the_step_mirrors_the_rxp_rule` rejects divergence.

# Examples

```rust
use crawrad_runtime_makepad::designer::ruler_step;

assert_eq!(ruler_step(1.0), 50.0);
```
