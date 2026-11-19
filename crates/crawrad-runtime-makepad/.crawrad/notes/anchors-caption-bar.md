---
id: anchors-caption-bar
kind: doc
target_file: src/anchors.rs
line_hint: 19
author: 
created: 
title: "CAPTION_BAR"
tags: []
signature: "pub const CAPTION_BAR : f64 = 27.0"
---

The height of Makepad's fallback window caption bar.

The height of the caption bar Makepad draws inside the window when the
platform does not (`makepad-widgets-1.0.0/src/window.rs:29`). It comes out
of the window's inner size before the form gets the rest.

# Examples

```rust
use crawrad_runtime_makepad::anchors::CAPTION_BAR;

assert_eq!(CAPTION_BAR, 27.0);
```
