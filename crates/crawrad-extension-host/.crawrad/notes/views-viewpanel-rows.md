---
id: views-viewpanel-rows
kind: doc
target_file: src/views.rs
line_hint: 270
author: 
created: 
title: "ViewPanel::rows"
tags: []
signature: "pub fn rows (& self , views : & [ExtensionView]) -> Vec < ViewRow >"
---

Computes visible rows for views applying the current filter.

The rows for `views` under the filter. While filtering, every branch
that shows is open: what matched must be in sight. An item that
matches brings its whole subtree; one that does not stays only as the
way to a descendant that does. With no view, nothing installed and no
filter, a single hint row says how to install one.

# Examples

```rust
use crawrad_extension_host::views::ViewPanel;

let panel = ViewPanel::default();
assert_eq!(panel.rows(&[]).len(), 1, "only the install hint");
```
