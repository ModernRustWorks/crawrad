---
id: components-main-menu-draw-bar
kind: doc
target_file: src/components/main_menu.rs
line_hint: 25
author: 
created: 
title: "draw_bar"
tags: []
signature: "pub fn draw_bar (p : & mut Brush , menu : & MakepadCtl , form : RectIr , scene : & Scene) -> bool"
---

Draws the MainMenu bar along the top of the form.

**The MainMenu's bar** along the top of the `form`: one entry per
top-level `MenuItem`, a `-` caption or a hidden item left out. Nothing
while the menu has no items.

# Examples

```rust,no_run
// MakepadCtl and Brush require the Makepad canvas runtime.
use crawrad_ir::Rect;
use crawrad_runtime_makepad::MakepadCtl;
use crawrad_runtime_makepad::canvas::Brush;
use crawrad_runtime_makepad::components::Scene;
use crawrad_runtime_makepad::components::main_menu::draw_bar;

fn example(brush: &mut Brush, menu: &MakepadCtl, form: Rect, scene: &Scene) {
    let _ = draw_bar(brush, menu, form, scene);
}
```
