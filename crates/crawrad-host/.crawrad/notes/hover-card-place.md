---
id: hover-card-place
kind: doc
target_file: src/hover_card.rs
line_hint: 90
author: 
created: 
title: "place"
tags: []
signature: "pub fn place (anchor : Anchor , size : (f64 , f64) , window : (f64 , f64 , f64 , f64)) -> (f64 , f64)"
---

Calculates coordinates for placing the card on screen.

Where a card of `size` (width, height) goes for `anchor` inside `window`
(x, y, width, height): **above** the line when it fits, else below; never
out of the window.

# Examples

```rust
use crawrad_host::hover_card::{place, Anchor};

let pos = place(Anchor { x: 10.0, y: 50.0, line_height: 12.0 }, (100.0, 30.0), (0.0, 0.0, 800.0, 600.0));
assert!(pos.1 >= 0.0);
```
