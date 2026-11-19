---
id: view-tool-panel-shown
kind: doc
target_file: src/view_tool.rs
line_hint: 116
author: 
created: 
title: "panel_shown"
tags: []
signature: "pub fn panel_shown (panel : Panel) -> Answer"
---

The answer when a panel was switched.

# Examples

```rust
use crawrad_mcp::view_tool::panel_shown;
use crawrad_actions::Panel;

let (result, status) = panel_shown(Panel::Explorer);
assert_eq!(status, "applied");
```
