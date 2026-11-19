---
id: components-bar-scrollbarstate-arrow
kind: doc
target_file: src/components/bar.rs
line_hint: 165
author: 
created: 
title: "ScrollBarState::arrow"
tags: []
signature: "pub fn arrow (self) -> [f32 ; 4]"
---

The colour of the two arrows of this bar.

No new token: the live arrow is the `MAKEPAD_TEXT` of an enabled
control and the dead one is the `MAKEPAD_TEXT_DIM`, which already means
*disabled* in this palette (it mirrors the RXP's `CTL_TEXTO_FRACO`) and
is what [`crate::components::text_color`] hands out for the very
same reason.

# Examples

```rust
use crawrad_runtime_makepad::components::bar::ScrollBarState;
use crawrad_runtime_makepad::components::comum::MAKEPAD_TEXT;

assert_eq!(ScrollBarState::Active.arrow(), MAKEPAD_TEXT);
```
