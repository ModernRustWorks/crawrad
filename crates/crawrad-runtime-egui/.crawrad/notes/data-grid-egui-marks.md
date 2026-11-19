---
id: data-grid-egui-marks
kind: doc
target_file: src/data_grid.rs
line_hint: 20
author: 
created: 
title: "EGUI_MARKS"
tags: []
signature: "pub const EGUI_MARKS : Marks = Marks"
---

Sort and group indicator marks supported by egui fonts.

The marks egui's default fonts can draw: they have no `▲`/`▼`, only the
`⏶`/`⏷` of the icon font egui itself uses for its arrows.

# Examples

```rust
use crawrad_runtime_egui::data_grid::EGUI_MARKS;

assert_eq!(EGUI_MARKS.ascending, " ⏶");
assert_eq!(EGUI_MARKS.descending, " ⏷");
assert_eq!(EGUI_MARKS.group_open, "⏷ ");
```
