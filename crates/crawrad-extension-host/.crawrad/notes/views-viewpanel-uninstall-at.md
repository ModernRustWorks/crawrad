---
id: views-viewpanel-uninstall-at
kind: doc
target_file: src/views.rs
line_hint: 213
author: 
created: 
title: "ViewPanel::uninstall_at"
tags: []
signature: "pub fn uninstall_at (& self , row : usize) -> Option < String >"
---

The extension whose uninstall button is on row `row`.

# Examples

```rust
use crawrad_extension_host::views::ViewPanel;

let panel = ViewPanel::default();
assert!(panel.uninstall_at(0).is_none());
```
