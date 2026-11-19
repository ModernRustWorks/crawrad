---
id: data-grid-compute-fetch-request
kind: doc
target_file: src/data_grid.rs
line_hint: 62
author: 
created: 
title: "compute_fetch_request"
tags: []
signature: "pub fn compute_fetch_request (visible_first : usize , requested_first : usize , page_size : usize ,) -> Option < (usize , usize) >"
---

Computes the window range to request when the visible range changes.

Computes the window range to request from the background source when the visible range changes.

Returns `Some((first, count))` if a fetch should be dispatched, or `None` if the visible
start matches the already requested window start.

# Examples

```rust
use crawrad_runtime_egui::data_grid::compute_fetch_request;

assert_eq!(compute_fetch_request(50, 0, 200), Some((50, 200)));
assert_eq!(compute_fetch_request(0, 0, 200), None);
```
