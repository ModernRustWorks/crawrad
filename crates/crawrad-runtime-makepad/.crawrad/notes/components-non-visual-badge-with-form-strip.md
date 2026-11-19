---
id: components-non-visual-badge-with-form-strip
kind: doc
target_file: src/components/non_visual.rs
line_hint: 82
author: 
created: 
title: "Badge::with_form_strip"
tags: []
signature: "pub const fn with_form_strip (self , strip : FormStrip) -> Self"
---

The same badge, and a strip along the form it paints besides.

# Examples

```rust
use crawrad_runtime_makepad::components::non_visual::Badge;

let badge = Badge::for_class("MainMenu").with_form_strip(|_, _, _, _| true);
```
