---
id: progress-card-progress-bar-width
kind: doc
target_file: src/progress_card.rs
line_hint: 103
author: 
created: 
title: "progress_bar_width"
tags: []
signature: "pub fn progress_bar_width (area_width : f64 , fraction : Option < f64 >) -> Option < f64 >"
---

Computes progress bar width from available width and optional fraction.

Computes the pixel width of the progress bar based on available width and
optional fraction.

# Examples

```rust
use crawrad_runtime_makepad::progress_card::progress_bar_width;

assert_eq!(progress_bar_width(100.0, Some(0.5)), Some(50.0));
assert_eq!(progress_bar_width(100.0, None), None);
```
