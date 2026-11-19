---
id: components-status-bar-status-panel
kind: doc
target_file: src/components/status_bar.rs
line_hint: 30
author: 
created: 
title: "status_panel"
tags: []
signature: "pub fn status_panel () -> ComponentDesc"
---

An individual segment panel of a `StatusBar`.

One panel of a `StatusBar` (#458): its text, its width in points, and
where the text sits in it.

# Examples

```rust
use crawrad_registry::status_panel;

let desc = status_panel();
assert_eq!(desc.class, "StatusPanel");
```
