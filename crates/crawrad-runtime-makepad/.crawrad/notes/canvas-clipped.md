---
id: canvas-clipped
kind: doc
target_file: src/canvas.rs
line_hint: 520
author: 
created: 
title: "Clipped"
tags: []
signature: "pub struct Clipped < T >"
---

Something the canvas paints, plus the rectangle that clips it.

Both in design units, because the conversion to screen happens once, in the
painter (ADR 0019).

The point of carrying the two together is that they must not drift apart:
while the clip was decided in one place and the ink in another, the handles
of a control cut in half by its container were still drawn whole, floating
over a piece of component that is not on screen — with
`Designer::hit_handle` refusing the drag. That is the interface lying, and
it is the defect this house has already paid for twice.

# Examples

```rust
use crawrad_runtime_makepad::canvas::Clipped;

let clipped = Clipped::free(42);
assert_eq!(clipped.item, 42);
assert_eq!(clipped.clip, None);
```
