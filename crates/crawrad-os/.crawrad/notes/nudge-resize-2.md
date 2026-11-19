---
id: nudge-resize-2
kind: doc
target_file: src/lib.rs
line_hint: 259
author: 
created: 
title: "nudge_resize"
tags: []
signature: "pub fn nudge_resize () -> bool"
---

Makes the UI framework re-read this window's geometry — see the Windows arm.

# Examples

```rust
use crawrad_os::nudge_resize;

assert!(!nudge_resize());
```
