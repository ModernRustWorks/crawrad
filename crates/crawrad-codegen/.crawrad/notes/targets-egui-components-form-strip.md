---
id: targets-egui-components-form-strip
kind: doc
target_file: src/targets/egui/components/mod.rs
line_hint: 203
author: 
created: 
title: "form_strip"
tags: []
signature: "pub fn form_strip (class : & str) -> bool"
---

Indicates whether a class paints a strip along the form.

Does this class paint a strip along the form instead of a control?

# Examples

```rust
use crawrad_codegen::targets::egui::components::form_strip;

assert!(form_strip("MainMenu"));
assert!(!form_strip("Button"));
```
