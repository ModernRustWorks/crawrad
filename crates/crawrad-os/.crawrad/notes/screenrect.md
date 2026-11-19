---
id: screenrect
kind: doc
target_file: src/lib.rs
line_hint: 53
author: 
created: 
title: "ScreenRect"
tags: []
signature: "pub struct ScreenRect"
---

A rectangle on the desktop, in **physical** pixels.

Physical and not logical, because that is the only unit Win32 speaks: the
process is DPI aware (`makepad-platform` calls `SetProcessDpiAwareness` at
startup — `os/windows/win32_app.rs:601-615`), so every coordinate crossing
this boundary is a real device pixel. Converting to the logical units a
framework wants is the caller's job, and it is the caller who knows which
DPI factor applies.

# Examples

```rust
use crawrad_os::ScreenRect;

let rect = ScreenRect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 };
assert_eq!(rect.right(), 800.0);
```
