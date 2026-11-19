---
id: scrolling-scroll-bar-box-for-kind
kind: doc
target_file: src/scrolling.rs
line_hint: 100
author: 
created: 
title: "scroll_bar_box_for_kind"
tags: []
signature: "pub fn scroll_bar_box_for_kind (rect : crate :: Rect , kind : & str) -> Option < crate :: Rect >"
---

Calculates the box a `ScrollBar` takes when its `Kind` is set.

**The box a `ScrollBar` takes when its `Kind` is set** (#451): the
canvas and both targets read the orientation from the box's shape, so, as
in the classic RAD IDE, turning `Kind` swaps the width and the height when
the shape says the other one. `None` when the box already agrees.

# Examples

```rust
use crawrad_ir::scrolling::scroll_bar_box_for_kind;
use crawrad_ir::Rect;

let rect = Rect::new(0.0, 0.0, 100.0, 20.0);
let swapped = scroll_bar_box_for_kind(rect, "sbVertical");
assert_eq!(swapped, Some(Rect::new(0.0, 0.0, 20.0, 100.0)));
```
