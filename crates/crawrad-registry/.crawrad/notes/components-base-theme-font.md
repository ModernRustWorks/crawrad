---
id: components-base-theme-font
kind: doc
target_file: src/components/base.rs
line_hint: 67
author: 
created: 
title: "theme_font"
tags: []
signature: "pub fn theme_font () -> crawrad_ir :: Font"
---

Returns the default theme font for a control or form.

The font a control or form starts with: no name and size 0 mean *the
theme's own* (#433), so a form nobody styled looks as it always did.

# Examples

```rust
use crawrad_registry::theme_font;

let font = theme_font();
assert_eq!(font.size, 0.0);
```
