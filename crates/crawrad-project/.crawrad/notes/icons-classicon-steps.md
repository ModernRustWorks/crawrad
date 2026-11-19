---
id: icons-classicon-steps
kind: doc
target_file: src/icons.rs
line_hint: 559
author: 
created: 
title: "ClassIcon::steps"
tags: []
signature: "pub fn steps (self) -> & 'static [crawrad_ir :: icon :: IconStep]"
---

Drawing instructions as data for this class icon kind.

The drawing, as data — the same steps the `CrawradClassIcon` shader paints for this kind.

# Examples

```rust
use crawrad_project::icons::ClassIcon;

assert!(!ClassIcon::Button.steps().is_empty());
```
