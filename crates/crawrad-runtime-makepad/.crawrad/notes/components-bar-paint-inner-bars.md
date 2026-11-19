---
id: components-bar-paint-inner-bars
kind: doc
target_file: src/components/bar.rs
line_hint: 250
author: 
created: 
title: "paint_inner_bars"
tags: []
signature: "pub fn paint_inner_bars (p : & mut Brush , r : Rect , bars : Bars , position : f32) -> Rect"
---

Paints a container's inner bars and returns the content rectangle.

Paints a container's INNER bars and returns the rectangle left over for the
content.

The order matters and it is Windows's: the vertical bar goes all the way
down, the horizontal one stops before it, and the little corner square stays
empty.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::Transformacao;
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::components::bar::{paint_inner_bars, Bars};

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 100.0) };
let content = paint_inner_bars(&mut brush, r, Bars::NONE, 0.0);
assert_eq!(content, r);
```
