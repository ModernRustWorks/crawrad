---
id: splash-window-frame
kind: doc
target_file: src/splash_window.rs
line_hint: 36
author: 
created: 
title: "Frame"
tags: []
signature: "pub struct Frame"
---

What to paint over the picture, fills first.

# Examples

```rust
use crawrad_os::splash_window::Frame;

let frame = Frame::default();
assert!(frame.fills.is_empty());
```
