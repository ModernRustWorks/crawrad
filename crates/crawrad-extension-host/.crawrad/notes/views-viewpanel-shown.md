---
id: views-viewpanel-shown
kind: doc
target_file: src/views.rs
line_hint: 221
author: 
created: 
title: "ViewPanel::shown"
tags: []
signature: "pub fn shown (& self) -> Vec < ViewRow >"
---

The rows of the views as last seen.

# Examples

```rust
use crawrad_extension_host::views::ViewPanel;

let panel = ViewPanel::default();
assert_eq!(panel.shown().len(), 1, "only the install hint");
```
