---
id: string-grid-crawradstringgridref-set-grid
kind: doc
target_file: src/string_grid.rs
line_hint: 272
author: 
created: 
title: "CrawradStringGridRef::set_grid"
tags: []
signature: "pub fn set_grid (& self , cx : & mut Cx , layout : GridLayout , cells : & Cells , grid_lines : bool , editing : bool ,)"
---

Synchronizes grid geometry, cell contents, line visibility, and editing.

Synchronizes the string grid geometry, cell contents, line visibility, and editing capability.
Redraws only if any value changed.

# Examples

```rust,no_run
// Makepad Cx and grid layout require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime::string_grid::{Cells, GridLayout};
use crawrad_runtime_makepad::string_grid::CrawradStringGridRef;

fn update(cx: &mut Cx, grid: &CrawradStringGridRef) {
    grid.set_grid(cx, GridLayout::default(), &Cells::default(), true, false);
}
```
