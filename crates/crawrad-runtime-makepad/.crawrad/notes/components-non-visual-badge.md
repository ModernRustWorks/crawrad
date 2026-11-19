---
id: components-non-visual-badge
kind: doc
target_file: src/components/non_visual.rs
line_hint: 67
author: 
created: 
title: "Badge"
tags: []
signature: "pub struct Badge"
---

The drawing of ONE non-visual — the same one for all four.

It holds the class because dispatch compares it (`classe()`), and nothing
else: there is no per-class branch in here, and adding a new non-visual
means creating its file with a constant and citing it in the catalog.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::non_visual::Badge;

let badge = Badge::for_class("Timer");
assert_eq!(badge.class_name(), "Timer");
```
