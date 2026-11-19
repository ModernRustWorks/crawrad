---
id: components-timer-timer
kind: doc
target_file: src/components/timer.rs
line_hint: 5
author: 
created: 
title: "timer"
tags: []
signature: "pub fn timer () -> ComponentDesc"
---

The recurring timer non-visual component.

`Timer`: the timer. The first non-visual of the project.

Note what is **not** here: `Visible`, `Anchors`, `Align`, `TabOrder`.
Whatever has no window has none of that, and the
[`non_visual_props`] step is what says this once and for all.

# Examples

```rust
use crawrad_registry::timer;

let desc = timer();
assert_eq!(desc.class, "Timer");
```
