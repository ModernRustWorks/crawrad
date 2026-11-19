---
id: string-grid-crawradstringgridref-cell-edited
kind: doc
target_file: src/string_grid.rs
line_hint: 295
author: 
created: 
title: "CrawradStringGridRef::cell_edited"
tags: []
signature: "pub fn cell_edited (& self , actions : & Actions) -> Option < (usize , usize , String) >"
---

Returns edited cell coordinates and text `(col, row, text)` if edited.

Returns the edited cell coordinate and new text `(col, row, text)` if a cell edit occurred.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::string_grid::CrawradStringGridRef;

fn check(grid: &CrawradStringGridRef, actions: &Actions) {
    let _edited = grid.cell_edited(actions);
}
```
