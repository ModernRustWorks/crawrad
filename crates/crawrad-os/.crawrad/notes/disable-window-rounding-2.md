---
id: disable-window-rounding-2
kind: doc
target_file: src/lib.rs
line_hint: 234
author: 
created: 
title: "disable_window_rounding"
tags: []
signature: "pub fn disable_window_rounding () -> bool"
---

Sets the main window to never have rounded corners on non-Windows.

Sets the main window to never have rounded corners: non-Windows platforms.

# Examples

```rust
use crawrad_os::disable_window_rounding;

assert!(!disable_window_rounding());
```
