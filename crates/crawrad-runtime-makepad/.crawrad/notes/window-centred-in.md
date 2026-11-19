---
id: window-centred-in
kind: doc
target_file: src/window.rs
line_hint: 20
author: 
created: 
title: "centred_in"
tags: []
signature: "pub fn centred_in (work : ScreenRect , outer : DVec2 , dpi : f64) -> DVec2"
---

Computes the logical position to center a window within a work area.

The logical point that puts a window `outer` logical points big, at
`dpi`, in the middle of `work` (physical pixels). A window larger than the
area starts at the area's corner, as the classic form does, so its caption
stays reachable.

# Examples

```rust
use makepad_widgets::dvec2;
use crawrad_os::ScreenRect;
use crawrad_runtime_makepad::window::centred_in;

let work = ScreenRect { x: 0.0, y: 0.0, width: 1920.0, height: 1032.0 };
let at = centred_in(work, dvec2(640.0, 420.0), 1.0);
assert_eq!(at, dvec2(640.0, 306.0));
```
