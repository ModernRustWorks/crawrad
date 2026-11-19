---
id: screenrect-of-window
kind: doc
target_file: src/lib.rs
line_hint: 67
author: 
created: 
title: "ScreenRect::of_window"
tags: []
signature: "pub fn of_window ([x , y] : [f64 ; 2] , [width , height] : [f64 ; 2] , dpi : f64) -> Self"
---

The window rectangle as the system sees it, in physical pixels.

**The window rectangle as the system sees it**, in physical pixels,
from the framework's window geometry.

It is the rectangle Windows compares against the taskbar, so it is the
one worth asking `fits_in` about and the one to hand back on restore.

The mixed arithmetic is the framework's asymmetry, and this is the only
place that pays it: `position` comes back physical (`get_position`
returns `GetWindowRect` untouched — `win32_window.rs:906-912`) while
`outer_size` comes back logical (`:920-927` divides by the DPI).

A `dpi` of zero counts as one: it would collapse the window to nothing,
and `WindowGeom::default()` has exactly that zero
(`makepad-platform-1.0.0/src/event/window.rs:11-20` derives `Default`) —
a geometry read before the window is created is a real default.

# Examples

```rust
use crawrad_os::ScreenRect;

let rect = ScreenRect::of_window([100.0, 100.0], [400.0, 300.0], 1.5);
assert_eq!(rect.width, 600.0);
```
