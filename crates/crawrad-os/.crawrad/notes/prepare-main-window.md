---
id: prepare-main-window
kind: doc
target_file: src/lib.rs
line_hint: 244
author: 
created: 
title: "prepare_main_window"
tags: []
signature: "pub fn prepare_main_window () -> bool"
---

Readies the main window: square corners, and a maximize that spares the taskbar.

Disables DWM window rounding and installs a subclass window procedure answering
`WM_GETMINMAXINFO` so that native maximize (Win+Up, double-click, `SW_MAXIMIZE`)
fits within the monitor's work area instead of covering the taskbar.

# Examples

```rust
use crawrad_os::prepare_main_window;

let prepared = prepare_main_window();
assert!(prepared || !prepared);
```
