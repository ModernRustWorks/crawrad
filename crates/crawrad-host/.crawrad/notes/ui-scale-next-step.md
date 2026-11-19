---
id: ui-scale-next-step
kind: doc
target_file: src/ui_scale.rs
line_hint: 11
author: 
created: 
title: "next_step"
tags: []
signature: "pub fn next_step (current : f64 , inward : bool) -> f64"
---

The neighbour of `current` in [`STEPS`], in the direction asked for.

It clamps at the ends instead of wrapping: someone holding `Ctrl -` expects
to stop at the smallest, not to jump back to the biggest.

# Examples

```rust
use crawrad_host::ui_scale::next_step;

assert_eq!(next_step(1.0, true), 1.1);
assert_eq!(next_step(1.0, false), 0.9);
```
