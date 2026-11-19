---
id: segment-list-rect-of
kind: doc
target_file: src/segment_list.rs
line_hint: 249
author: 
created: 
title: "rect_of"
tags: []
signature: "pub fn rect_of (r : Rect) -> [f64 ; 4]"
---

A Makepad rect as `[x, y, w, h]`, the shape MCP views and hit tests read.

# Examples

```
use crawrad_runtime_makepad::makepad_widgets::{Rect, dvec2};
use crawrad_runtime_makepad::segment_list::rect_of;

let r = Rect { pos: dvec2(1.0, 2.0), size: dvec2(3.0, 4.0) };
assert_eq!(rect_of(r), [1.0, 2.0, 3.0, 4.0]);
```
