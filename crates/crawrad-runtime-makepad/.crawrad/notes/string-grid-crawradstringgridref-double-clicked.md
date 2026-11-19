---
id: string-grid-crawradstringgridref-double-clicked
kind: doc
target_file: src/string_grid.rs
line_hint: 322
author: 
created: 
title: "CrawradStringGridRef::double_clicked"
tags: []
signature: "pub fn double_clicked (& self , actions : & Actions) -> bool"
---

Returns whether the grid was double-clicked.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::string_grid::CrawradStringGridRef;

fn check(grid: &CrawradStringGridRef, actions: &Actions) {
    let _dbl = grid.double_clicked(actions);
}
```
