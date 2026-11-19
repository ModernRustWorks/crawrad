---
id: data-grid-crawraddatagridref-set-timing
kind: doc
target_file: src/data_grid.rs
line_hint: 799
author: 
created: 
title: "CrawradDataGridRef::set_timing"
tags: []
signature: "pub fn set_timing (& self , timing : crawrad_component :: BusyTiming)"
---

Sets the anti-flicker timing for the loading overlay.

# Examples

```rust,no_run
// WidgetRef requires the Makepad application runtime.
use crawrad_component::BusyTiming;
use crawrad_runtime_makepad::data_grid::CrawradDataGridRef;

fn example(grid: &CrawradDataGridRef) {
    grid.set_timing(BusyTiming::default());
}
```
