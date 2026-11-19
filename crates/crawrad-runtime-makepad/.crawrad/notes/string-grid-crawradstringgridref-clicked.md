---
id: string-grid-crawradstringgridref-clicked
kind: doc
target_file: src/string_grid.rs
line_hint: 317
author: 
created: 
title: "CrawradStringGridRef::clicked"
tags: []
signature: "pub fn clicked (& self , actions : & Actions) -> bool"
---

Returns whether the grid was clicked.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::string_grid::CrawradStringGridRef;

fn check(grid: &CrawradStringGridRef, actions: &Actions) {
    let _clicked = grid.clicked(actions);
}
```
