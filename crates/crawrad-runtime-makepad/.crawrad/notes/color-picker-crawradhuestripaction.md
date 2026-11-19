---
id: color-picker-crawradhuestripaction
kind: doc
target_file: src/color_picker.rs
line_hint: 248
author: 
created: 
title: "CrawradHueStripAction"
tags: []
signature: "pub enum CrawradHueStripAction"
---

Actions emitted by the hue strip widget.

# Examples

```rust
use crawrad_runtime_makepad::color_picker::CrawradHueStripAction;

let a = CrawradHueStripAction::Picked(0.5);
assert!(matches!(a, CrawradHueStripAction::Picked(0.5)));
```
