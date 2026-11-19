---
id: edit-align-x-of
kind: doc
target_file: src/edit.rs
line_hint: 7
author: 
created: 
title: "align_x_of"
tags: []
signature: "pub fn align_x_of (alignment : & str) -> f64"
---

Returns the Makepad horizontal alignment float for an alignment name.

Returns the Makepad horizontal alignment float (`x` of `label_align`)
corresponding to an alignment enum name (`taLeftJustify`, `taCenter`, `taRightJustify`).

# Examples

```rust
use crawrad_runtime_makepad::edit::align_x_of;

assert_eq!(align_x_of("taCenter"), 0.5);
assert_eq!(align_x_of("taRightJustify"), 1.0);
assert_eq!(align_x_of("taLeftJustify"), 0.0);
```
