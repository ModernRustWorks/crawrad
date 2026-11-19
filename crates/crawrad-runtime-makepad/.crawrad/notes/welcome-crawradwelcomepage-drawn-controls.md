---
id: welcome-crawradwelcomepage-drawn-controls
kind: doc
target_file: src/welcome.rs
line_hint: 348
author: 
created: 
title: "CrawradWelcomePage::drawn_controls"
tags: []
signature: "pub fn drawn_controls (& self , cx : & Cx) -> Vec < (String , Rect) >"
---

Button and row rects, for the MCP's `click` by name.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use crawrad_runtime_makepad::welcome::CrawradWelcomePage;

# fn demo(page: &CrawradWelcomePage, cx: &makepad_widgets::Cx) {
let controls = page.drawn_controls(cx);
assert!(!controls.is_empty());
# }
```
