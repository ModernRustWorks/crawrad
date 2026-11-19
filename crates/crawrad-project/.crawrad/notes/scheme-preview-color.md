---
id: scheme-preview-color
kind: doc
target_file: src/scheme_preview.rs
line_hint: 146
author: 
created: 
title: "color"
tags: []
signature: "pub fn color (prefs : & crate :: workspace :: Preferences , role : Role) -> [f32 ; 4]"
---

Resolves the RGBA color for a given preview role from preferences.

Resolves the RGBA color `[f32; 4]` for a given preview role according to active preferences.

# Examples

```rust
use crawrad_project::scheme_preview::{color, Role};
use crawrad_project::workspace::Preferences;

let prefs = Preferences::default();
let c = color(&prefs, Role::Keyword);
assert_eq!(c.len(), 4);
```
