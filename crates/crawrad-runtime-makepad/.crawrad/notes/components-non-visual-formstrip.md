---
id: components-non-visual-formstrip
kind: doc
target_file: src/components/non_visual.rs
line_hint: 73
author: 
created: 
title: "FormStrip"
tags: []
signature: "pub type FormStrip = fn (& mut Brush , & MakepadCtl , crawrad_ir :: Rect , & Scene) -> bool"
---

A strip the class paints along the form — the `MainMenu` bar (#538).

# Examples

```rust
use crawrad_runtime_makepad::components::non_visual::FormStrip;

fn dummy_strip(
    _: &mut crawrad_runtime_makepad::canvas::Brush,
    _: &crawrad_runtime_makepad::MakepadCtl,
    _: crawrad_ir::Rect,
    _: &crawrad_runtime_makepad::components::Scene,
) -> bool {
    true
}

let strip: FormStrip = dummy_strip;
assert!(strip as usize != 0);
```
