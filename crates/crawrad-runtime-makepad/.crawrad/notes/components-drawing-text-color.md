---
id: components-drawing-text-color
kind: doc
target_file: src/components/drawing.rs
line_hint: 30
author: 
created: 
title: "text_color"
tags: []
signature: "pub fn text_color (c : & MakepadCtl) -> [f32 ; 4]"
---

A control's text colour: live when enabled, dimmed when not.

It comes from here, and not from each file, because it is a rule of the THEME
and not of the component — and a colour on the canvas always comes from a
token.

# Examples

```rust,no_run
// MakepadCtl requires the Makepad runtime.
use crawrad_runtime_makepad::MakepadCtl;
use crawrad_runtime_makepad::components::drawing::text_color;

fn example(ctl: &MakepadCtl) {
    let color = text_color(ctl);
    assert_eq!(color.len(), 4);
}
```
