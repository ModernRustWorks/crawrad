---
id: views-viewpanel
kind: doc
target_file: src/views.rs
line_hint: 161
author: 
created: 
title: "ViewPanel"
tags: []
signature: "pub struct ViewPanel"
---

The Extensions panel state and row hierarchy.

The panel's own state: the views as last seen, the filter, what the user
closed, and where each drawn row is (row, x, y, w, h) — what MCP gives a
test to click.

# Examples

```rust
use crawrad_extension_host::views::ViewPanel;

let panel = ViewPanel::default();
assert!(panel.views.is_empty());
```
