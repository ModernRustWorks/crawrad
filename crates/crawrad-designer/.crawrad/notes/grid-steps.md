---
id: grid-steps
kind: doc
target_file: src/lib.rs
line_hint: 157
author: 
created: 
title: "GRID_STEPS"
tags: []
signature: "pub const GRID_STEPS : [f32 ; 6] = [2.0 , 4.0 , 5.0 , 8.0 , 10.0 , 16.0]"
---

The grid sizes the designer bar's `[-]` and `[+]` step through (#564).

The Options accept any size from 2 to 64; the bar offers the ones a form is
laid out on, the way its Zoom offers its stops.

# Examples

```rust
use crawrad_designer::GRID_STEPS;

assert_eq!(GRID_STEPS[0], 2.0);
```
