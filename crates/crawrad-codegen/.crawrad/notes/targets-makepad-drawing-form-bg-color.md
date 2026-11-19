---
id: targets-makepad-drawing-form-bg-color
kind: doc
target_file: src/targets/makepad/drawing.rs
line_hint: 7
author: 
created: 
title: "FORM_BG_COLOR"
tags: []
signature: "pub const FORM_BG_COLOR : & str = \"#242424\""
---

Default window background color for Makepad apps.

# Examples

```rust
use crawrad_codegen::targets::makepad::drawing::FORM_BG_COLOR;

assert!(FORM_BG_COLOR.starts_with('#'));
```
