---
id: place-main-window
kind: doc
target_file: src/lib.rs
line_hint: 164
author: 
created: 
title: "place_main_window"
tags: []
signature: "pub fn place_main_window (rect : ScreenRect) -> bool"
---

Puts this process's window where `rect` says, notifying the UI framework.

**Puts this process's window where `rect` says, and makes the UI framework
notice.**

# Why this is here and not a call to the framework

Makepad publishes `CxOsOp::RepositionWindow` and `CxOsOp::ResizeWindow`,
and they do move the window — the frame really goes where it is told. What
they do NOT do is tell the framework's own renderer, and the owner
photographed the result: the frame filled the screen and the drawing stayed
the size it was, with black where the window had grown.

The cause is measured, and it is one line of the framework:

```text
// makepad-platform-1.0.0/src/os/windows/win32_app.rs:294-304
pub fn do_callback(event: Win32Event) {
    let cb = with_win32_app(|app| app.event_callback.take());   // <- TAKE
    if let Some(mut callback) = cb { … }                        // <- else: dropped
}
```

Those two ops reach `MoveWindow`, which delivers `WM_SIZE` **synchronously**
to the window procedure. The procedure answers it with `send_change_event`
→ `do_callback` — and at that instant the callback has already been taken,
because the ops are drained from inside it. The geometry change is dropped
in silence, so `D3d11Window::window_geom` keeps the old size and
`resize_buffers` returns early on `alloc_size == inner_size`
(`d3d11.rs:592-596`).

The framework's own maximize does not hit this, and the way it avoids it is
the technique copied here: it **posts** the message instead of letting
`MoveWindow` send it —

```text
// win32_window.rs:803-816
ShowWindow(self.hwnd, SW_MAXIMIZE);
PostMessageW(self.hwnd, WM_SIZE, WPARAM(0), LPARAM(0)).unwrap();
```

A posted message waits in the queue and is handled on the next turn of the
pump, when the callback is back in place. So this function does the move and
posts the same message, and the framework picks the new geometry up the way
it already knows how.

# `rect` is the WINDOW rectangle, in physical pixels

The whole frame, sizing border included — which is what Windows compares
against the taskbar, and what a work area is expressed in. There is no
logical-unit arithmetic here on purpose: the two unit traps that arithmetic
had (`WindowGeom::position` is physical while every setter is logical, and
`ResizeWindow` sets the client rect) belong to the framework's setters, and
this function does not use them.

# What it answers

`true` when the window was found and moved. `false` when this process has
no single main window to move — and the caller is expected to fall back to
the framework's own maximize rather than guess.

# Examples

```rust
use crawrad_os::{place_main_window, ScreenRect};

let placed = place_main_window(ScreenRect::default());
assert!(placed || !placed);
```
