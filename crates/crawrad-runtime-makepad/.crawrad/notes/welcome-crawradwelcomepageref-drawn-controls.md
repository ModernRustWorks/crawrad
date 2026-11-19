---
id: welcome-crawradwelcomepageref-drawn-controls
kind: doc
target_file: src/welcome.rs
line_hint: 423
author: 
created: 
title: "CrawradWelcomePageRef::drawn_controls"
tags: []
signature: "pub fn drawn_controls (& self , cx : & Cx) -> Vec < (String , Rect) >"
---

Drawn controls for the MCP.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use crawrad_runtime_makepad::welcome::CrawradWelcomePageRef;

# fn demo(page: &CrawradWelcomePageRef, cx: &makepad_widgets::Cx) {
let controls = page.drawn_controls(cx);
assert!(controls.is_empty());
# }
```
