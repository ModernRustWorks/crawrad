---
id: data-grid-crawraddatagridref-rec-no
kind: doc
target_file: src/data_grid.rs
line_hint: 819
author: 
created: 
title: "CrawradDataGridRef::rec_no"
tags: []
signature: "pub fn rec_no (& self) -> usize"
---

Returns the currently active selected row index (0-based).

# Examples

```rust,no_run
// WidgetRef requires the Makepad application runtime.
use crawrad_runtime_makepad::data_grid::CrawradDataGridRef;

fn example(grid: &CrawradDataGridRef) {
    let rec = grid.rec_no();
    assert_eq!(rec, 0);
}
```
