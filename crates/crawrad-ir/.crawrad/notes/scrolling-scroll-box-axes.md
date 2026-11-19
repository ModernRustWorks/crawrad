---
id: scrolling-scroll-box-axes
kind: doc
target_file: src/scrolling.rs
line_hint: 92
author: 
created: 
title: "scroll_box_axes"
tags: []
signature: "pub fn scroll_box_axes (auto_scroll : bool , horz_scroll_bar : bool , vert_scroll_bar : bool) -> Bars"
---

Which axes of a `ScrollBox` may scroll, from its properties.

**Which axes of a `ScrollBox` may scroll at all**, from its three
properties (#122).

`AutoScroll` turned off means the box never scrolls, whatever the content
does. `HorzScrollBar`/`VertScrollBar` turned off means that axis never
scrolls. An axis that may scroll still shows its bar only when the content
asks for it ([`bars_needed`]).

It lives here, and not in each side, because each side had read a
different subset: the designer and the Makepad app ignored `AutoScroll`,
and the egui app ignored the two axis properties.

# Examples

```rust
use crawrad_ir::scrolling::scroll_box_axes;

let bars = scroll_box_axes(true, true, false);
assert!(bars.horizontal);
assert!(!bars.vertical);
```
