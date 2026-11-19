---
id: explorer-action-toggle-legend
kind: doc
target_file: src/explorer.rs
line_hint: 105
author: 
created: 
title: "ACTION_TOGGLE_LEGEND"
tags: []
signature: "pub const ACTION_TOGGLE_LEGEND : & str = \"explorer.toggle_legend\""
---

The catalogue action that folds the legend.

A constant so the row that draws the chevron and the test that charges it
read the same string. Written twice, they would agree until the day one of
them was renamed — and the click would silently stop doing anything.

# Examples

```rust
use crawrad_project::explorer::ACTION_TOGGLE_LEGEND;

assert_eq!(ACTION_TOGGLE_LEGEND, "explorer.toggle_legend");
```
