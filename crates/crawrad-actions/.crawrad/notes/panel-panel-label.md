---
id: panel-panel-label
kind: doc
target_file: src/panel.rs
line_hint: 109
author: 
created: 
title: "Panel::label"
tags: []
signature: "pub fn label (self) -> & 'static str"
---

What the tab shows, in English like the rest of the interface.

# Examples

```rust
use crawrad_actions::Panel;

assert_eq!(Panel::Explorer.label(), "Explorer");
```
