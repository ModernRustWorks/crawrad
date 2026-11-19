---
id: window-centre
kind: doc
target_file: src/window.rs
line_hint: 29
author: 
created: 
title: "centre"
tags: []
signature: "pub fn centre (cx : & mut Cx , e : & WindowGeomChangeEvent) -> bool"
---

Centres a window on the work area of the monitor it opened on.

Centres the window of `e` on the work area of the monitor it opened on
(the taskbar left out). `false` when the system did not describe a monitor
(and on the platforms `crawrad-os` does not answer for), and the window stays
where it opened.

# Examples

```rust,no_run
// Makepad Cx and WindowGeomChangeEvent require the Makepad application runtime.
use makepad_widgets::{Cx, WindowGeomChangeEvent};
use crawrad_runtime_makepad::window::centre;

# fn demo(cx: &mut Cx, event: &WindowGeomChangeEvent) {
let _ = centre(cx, event);
# }
```
