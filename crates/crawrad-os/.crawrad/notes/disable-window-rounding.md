---
id: disable-window-rounding
kind: doc
target_file: src/lib.rs
line_hint: 201
author: 
created: 
title: "disable_window_rounding"
tags: []
signature: "pub fn disable_window_rounding () -> bool"
---

Sets the main window to NEVER have rounded corners on Windows 11.

On Windows 11 the DWM rounds the corners of top-level windows by default.
Calling `DwmSetWindowAttribute` with `DWMWA_WINDOW_CORNER_PREFERENCE = DWMWCP_DONOTROUND`
forces 100% square (90 degree) corners, honouring the classic RAD visual contract.

# Examples

```rust
use crawrad_os::disable_window_rounding;

let disabled = disable_window_rounding();
assert!(disabled || !disabled);
```
