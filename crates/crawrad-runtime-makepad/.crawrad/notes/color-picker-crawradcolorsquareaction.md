---
id: color-picker-crawradcolorsquareaction
kind: doc
target_file: src/color_picker.rs
line_hint: 121
author: 
created: 
title: "CrawradColorSquareAction"
tags: []
signature: "pub enum CrawradColorSquareAction"
---

Actions emitted by the color square widget.

# Examples

```rust
use crawrad_runtime_makepad::color_picker::CrawradColorSquareAction;

let a = CrawradColorSquareAction::Picked { s: 0.5, v: 0.5 };
assert!(matches!(a, CrawradColorSquareAction::Picked { .. }));
```
