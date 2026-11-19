---
id: handle-outset
kind: doc
target_file: src/lib.rs
line_hint: 306
author: 
created: 
title: "HANDLE_OUTSET"
tags: []
signature: "pub const HANDLE_OUTSET : f32 = HANDLE_SIDE / 2.0"
---

How far the handle sits **outside** the component rectangle.

Half a side: the inner edge of the square touches the edge of the component,
and none of it is covered.

Centered on the border — half inside, half outside, which is what the classic RAD IDE does —
it **eats the content**: on `PageControl` the top handles fell on top of the
tabs, and the middle one sat between `Page1` and `Page2`. Worse than ugly: tab
clicks are tested before selection (otherwise there would be no way to reach the second
page), so the top handle switched pages instead of resizing — the
control could not be resized from the top.

Outset also improves small components: on a 20pt high `Label`
centered handles covered almost everything, leaving little to grab and move.

# Examples

```rust
use crawrad_runtime::{HANDLE_OUTSET, HANDLE_SIDE};

assert_eq!(HANDLE_OUTSET, HANDLE_SIDE / 2.0);
```
