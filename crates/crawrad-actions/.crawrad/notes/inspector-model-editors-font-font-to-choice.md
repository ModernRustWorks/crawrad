---
id: inspector-model-editors-font-font-to-choice
kind: doc
target_file: src/inspector_model/editors/font.rs
line_hint: 319
author: 
created: 
title: "font_to_choice"
tags: []
signature: "pub fn font_to_choice (font : & Font) -> crawrad_os :: font :: FontChoice"
---

Converts a [`Font`] into an OS [`crawrad_os::font::FontChoice`].

Converts a document [`Font`] into an operating system [`crawrad_os::font::FontChoice`].

# Examples

```rust
use crawrad_actions::inspector_model::editors::font::font_to_choice;
use crawrad_ir::Font;

let font = Font::default();
let choice = font_to_choice(&font);
assert_eq!(choice.name, font.name);
```
