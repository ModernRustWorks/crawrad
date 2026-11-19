---
id: capture-main-window
kind: doc
target_file: src/lib.rs
line_hint: 372
author: 
created: 
title: "capture_main_window"
tags: []
signature: "pub fn capture_main_window (region : Option < [i32 ; 4] >) -> Option < Capture >"
---

Captures a region of the main window, or its whole client area.

`region` is in PHYSICAL pixels, relative to the client area's top-left
corner, and clamped to it. `None` when there is no single main window, the
clamped region is empty, or a GDI call fails. The window is drawn by
Direct3D, so only `PrintWindow` with `PW_RENDERFULLCONTENT` sees it — and
it does even when the window is covered by another one.

# Examples

```
// A test thread owns no window, so there is nothing to capture.
assert!(crawrad_os::capture_main_window(None).is_none());
```
