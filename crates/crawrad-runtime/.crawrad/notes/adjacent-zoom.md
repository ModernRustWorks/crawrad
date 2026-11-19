---
id: adjacent-zoom
kind: doc
target_file: src/lib.rs
line_hint: 337
author: 
created: 
title: "adjacent_zoom"
tags: []
signature: "pub fn adjacent_zoom (current : f32 , zoom_in : bool) -> f32"
---

The next step on the ladder, zooming in or out.

At the extremes, it returns the extreme itself: the command remains inert instead of
wrapping around. Wrapping around would save a click and cost the thing most expected
from zoom — that it does not jump from 400% to 25% because a finger lingered on the
button.

Accepts values outside the ladder (an old session, an agent sending `0.9`)
and returns the neighbor **in the requested direction**, rather than the closest: asking
to *zoom in* must not zoom out.

# Examples

```rust
use crawrad_runtime::adjacent_zoom;

assert_eq!(adjacent_zoom(1.0, true), 1.5);
assert_eq!(adjacent_zoom(1.0, false), 0.75);
```
