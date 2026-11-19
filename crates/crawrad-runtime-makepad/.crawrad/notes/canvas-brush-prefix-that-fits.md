---
id: canvas-brush-prefix-that-fits
kind: doc
target_file: src/canvas/brush.rs
line_hint: 1040
author: 
created: 
title: "prefix_that_fits"
tags: []
signature: "pub fn prefix_that_fits (text : & str , max : f64 , mut width_of : impl FnMut (& str) -> f64) -> & str"
---

Returns the longest prefix of `text` whose width fits in `max`.

The longest prefix of `text`, at a character boundary, whose width fits in
`max` — what is left visible when the running app clips a caption at its
box (#582). The whole text when it fits.

# Examples

```rust,ignore
// Not reachable from outside the crate: shown, not run.
use crate::canvas::brush::prefix_that_fits;

let s = prefix_that_fits("Hello World", 50.0, |t| t.len() as f64 * 10.0);
assert_eq!(s, "Hello");
```
