---
id: window-followed
kind: doc
target_file: src/window.rs
line_hint: 82
author: 
created: 
title: "followed"
tags: []
signature: "pub fn followed (event : & Event , maximized : & mut bool , restore_to : & mut Option < ScreenRect >) -> bool"
---

Keeps the title-bar Maximize/Restore button true to the screen.

**The button says "maximized" exactly when the window covers its monitor's work area.**

# What the owner saw

> *"IDE maximized on the button but not filling the screen"* (19/11)

The IDE's maximize is a placement over the work area, remembered in
[`IdeApp::maximizada`]. A window moved or sized by anything other than the
button — dragged, snapped, Win+arrow, another program — left that flag
behind: the button kept the Restore glyph on a window that no longer
covered the screen.

# How

On every `Event::WindowGeomChange`, the new outer rectangle is compared with
the work area of the monitor under its centre, 2 px of DPI rounding allowed.
Covering it means maximized, and Restore will go back to the rectangle the
window had just before; anything else means restored, and the remembered
"restore to" rectangle is dropped — the window already IS where it was put.

# Examples

```rust
use makepad_widgets::Event;
use crawrad_runtime_makepad::window::followed;

// Not a geometry change: nothing moves, nothing is reported.
let (mut maximized, mut restore_to) = (true, None);
assert!(!followed(&Event::Startup, &mut maximized, &mut restore_to));
assert!(maximized);
```
