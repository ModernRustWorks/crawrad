---
id: combo-box-thumb
kind: doc
target_file: src/combo_box.rs
line_hint: 160
author: 
created: 
title: "thumb"
tags: []
signature: "pub fn thumb (offset : usize , rows : usize , len : usize , track_h : f64) -> Option < (f64 , f64) >"
---

Calculates scroll bar thumb position and height inside track.

# Examples

```
use crawrad_runtime_makepad::combo_box::thumb;

assert_eq!(thumb(0, 8, 5, 100.0), None);
assert_eq!(thumb(0, 5, 10, 100.0), Some((0.0, 50.0)));
assert_eq!(thumb(5, 5, 10, 100.0), Some((50.0, 50.0)));
```
