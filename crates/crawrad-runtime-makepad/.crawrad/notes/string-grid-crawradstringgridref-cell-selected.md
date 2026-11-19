---
id: string-grid-crawradstringgridref-cell-selected
kind: doc
target_file: src/string_grid.rs
line_hint: 306
author: 
created: 
title: "CrawradStringGridRef::cell_selected"
tags: []
signature: "pub fn cell_selected (& self , actions : & Actions) -> Option < (usize , usize) >"
---

Returns clicked cell coordinates `(col, row)` if selected.

Returns the clicked cell coordinate `(col, row)` if a cell selection action occurred.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::string_grid::CrawradStringGridRef;

fn check(grid: &CrawradStringGridRef, actions: &Actions) {
    let _sel = grid.cell_selected(actions);
}
```
