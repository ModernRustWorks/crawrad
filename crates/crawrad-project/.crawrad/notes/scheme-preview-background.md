---
id: scheme-preview-background
kind: doc
target_file: src/scheme_preview.rs
line_hint: 178
author: 
created: 
title: "background"
tags: []
signature: "pub fn background (prefs : & crate :: workspace :: Preferences) -> [f32 ; 4]"
---

Resolves the editor background color according to active preferences.

# Examples

```rust
use crawrad_project::scheme_preview::background;
use crawrad_project::workspace::Preferences;

let prefs = Preferences::default();
assert_eq!(background(&prefs).len(), 4);
```
