---
id: palette-takeslook
kind: doc
target_file: src/palette.rs
line_hint: 13
author: 
created: 
title: "TakesLook"
tags: []
signature: "pub trait TakesLook"
---

Configures widget-specific styling when a new palette look is applied.

What a palette widget does with a new look besides storing it and
redrawing — set the font on its own draw fields, restyle its rows.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::list::ListLook;
use crawrad_runtime_makepad::palette::TakesLook;

struct Dummy;
impl TakesLook for Dummy {
    fn take_look(&mut self, _cx: &mut Cx, _look: ListLook) {}
}
```
