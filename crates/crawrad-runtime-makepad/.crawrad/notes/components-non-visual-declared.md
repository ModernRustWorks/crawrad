---
id: components-non-visual-declared
kind: doc
target_file: src/components/non_visual.rs
line_hint: 83
author: 
created: 
title: "DECLARED"
tags: []
signature: "pub static DECLARED : Badge = Badge :: for_class (DECLARED_CLASS)"
---

The badge drawing for catalogue non-visuals that bring no drawing of their own.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::non_visual::{DECLARED, DECLARED_CLASS};

assert_eq!(DECLARED.class_name(), DECLARED_CLASS);
assert!(!DECLARED.visual());
```
