---
id: axis
kind: doc
target_file: src/lib.rs
line_hint: 157
author: 
created: 
title: "Axis"
tags: []
signature: "pub enum Axis"
---

The axis along which a resize handle stretches.

Lives here, and not in the designer, for the same reason as `GuideLine`: it is
vocabulary that the designer **produces** and the adapter **consumes**, and the
adapter cannot depend on the designer — the arrow would end up duplicated, or
worse, inside a toolkit `match` within geometry.

# Examples

```rust
use crawrad_runtime::Axis;

let axis = Axis::Nwse;
assert_eq!(axis, Axis::Nwse);
```
