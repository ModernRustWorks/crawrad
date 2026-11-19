---
id: place-main-window-2
kind: doc
target_file: src/lib.rs
line_hint: 309
author: 
created: 
title: "place_main_window"
tags: []
signature: "pub fn place_main_window (_rect : ScreenRect) -> bool"
---

Puts this process's window where `rect` says — see the Windows arm.

Always `false` here, and the reason is the same as [`monitor_at`]'s: the
defect it works around is how Windows maximizes a `WS_POPUP`, and the
caller falls back to the framework's own maximize, which is already correct
on the other platforms.

# Examples

```rust
use crawrad_os::{place_main_window, ScreenRect};

assert!(!place_main_window(ScreenRect::default()));
```
