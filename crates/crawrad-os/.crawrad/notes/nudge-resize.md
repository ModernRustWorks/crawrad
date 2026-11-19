---
id: nudge-resize
kind: doc
target_file: src/lib.rs
line_hint: 241
author: 
created: 
title: "nudge_resize"
tags: []
signature: "pub fn nudge_resize () -> bool"
---

Makes the UI framework re-read this window's geometry, without moving it.

**Makes the UI framework re-read this window's geometry, without moving it.**

It posts the same `WM_SIZE` that [`place_main_window`] does and nothing
else. The measurement for why a POSTED message is the only one that lands
is in that function: `Win32App::do_callback` takes the event callback
before calling it, so anything delivered synchronously from inside the
callback is dropped in silence.

# What it is for

Scaling the whole interface. Makepad publishes `CxWindow::dpi_override`
(`makepad-platform-1.0.0/src/window.rs:256`) and applies it **when a
geometry change arrives** (`os/windows/windows.rs:115-117`), not when it is
written. Written alone it changes nothing; written and then nudged, the
framework recomputes the window's logical size, redraws every pass and the
whole IDE scales.

`false` when this process has no single main window — see
[`place_main_window`] for why the answer is exactly one.

# Examples

```rust
use crawrad_os::nudge_resize;

let nudged = nudge_resize();
assert!(nudged || !nudged);
```
