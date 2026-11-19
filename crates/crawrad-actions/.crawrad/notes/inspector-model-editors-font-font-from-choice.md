---
id: inspector-model-editors-font-font-from-choice
kind: doc
target_file: src/inspector_model/editors/font.rs
line_hint: 332
author: 
created: 
title: "font_from_choice"
tags: []
signature: "pub fn font_from_choice (choice : & crawrad_os :: font :: FontChoice , initial : & Font) -> Font"
---

Applies an OS [`crawrad_os::font::FontChoice`] back to a [`Font`].

Applies an operating system [`crawrad_os::font::FontChoice`] back to a [`Font`],
preserving alpha 0 (theme colour) when RGB is unchanged.

# Examples

```rust
use crawrad_actions::inspector_model::editors::font::{font_from_choice, font_to_choice};
use crawrad_ir::Font;

let font = Font::default();
let choice = font_to_choice(&font);
let restored = font_from_choice(&choice, &font);
assert_eq!(restored.name, font.name);
```
