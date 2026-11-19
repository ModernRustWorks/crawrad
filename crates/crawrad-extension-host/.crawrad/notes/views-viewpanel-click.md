---
id: views-viewpanel-click
kind: doc
target_file: src/views.rs
line_hint: 226
author: 
created: 
title: "ViewPanel::click"
tags: []
signature: "pub fn click (& mut self , row : usize) -> Option < ItemPick >"
---

Handles a click on a visible row.

A click on row `row` of [`Self::shown`]: a header opens or closes its
view, a branch its children; any item is handed back to be told to
its extension (and opened, when it points somewhere).

# Examples

```rust
use crawrad_extension_host::views::ViewPanel;

let mut panel = ViewPanel::default();
assert!(panel.click(0).is_none());
```
