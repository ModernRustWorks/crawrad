---
id: adjacent-grid
kind: doc
target_file: src/lib.rs
line_hint: 163
author: 
created: 
title: "adjacent_grid"
tags: []
signature: "pub fn adjacent_grid (grid : f32 , up : bool) -> f32"
---

Returns the grid size one step up or down from `grid`.

The grid size one step up or down from `grid`, or `grid` itself past the
last step. A size between two steps goes to the next one in that direction.

# Examples

```rust
use crawrad_designer::adjacent_grid;

assert_eq!(adjacent_grid(8.0, true), 10.0);
assert_eq!(adjacent_grid(8.0, false), 5.0);
```
