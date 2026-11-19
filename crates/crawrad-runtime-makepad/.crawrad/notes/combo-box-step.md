---
id: combo-box-step
kind: doc
target_file: src/combo_box.rs
line_hint: 175
author: 
created: 
title: "step"
tags: []
signature: "pub fn step (hovered : Option < usize > , key : KeyCode , rows : usize , len : usize) -> Option < usize >"
---

Computes new hovered index following keyboard navigation.

# Examples

```
use makepad_widgets::KeyCode;
use crawrad_runtime_makepad::combo_box::step;

assert_eq!(step(Some(0), KeyCode::ArrowDown, 5, 10), Some(1));
assert_eq!(step(Some(1), KeyCode::ArrowUp, 5, 10), Some(0));
assert_eq!(step(Some(0), KeyCode::End, 5, 10), Some(9));
```
