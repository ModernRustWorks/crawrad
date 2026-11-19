---
id: components-non-visual-badge-for-class
kind: doc
target_file: src/components/non_visual.rs
line_hint: 77
author: 
created: 
title: "Badge::for_class"
tags: []
signature: "pub const fn for_class (class : & 'static str) -> Self"
---

Creates a new badge for a class.

The badge for a class. `const` so that the class's file can declare a
`static` and the catalog can point at it.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::non_visual::Badge;

let badge = Badge::for_class("Timer");
assert_eq!(badge.class_name(), "Timer");
```
