---
id: scheme-preview-band
kind: doc
target_file: src/scheme_preview.rs
line_hint: 168
author: 
created: 
title: "band"
tags: []
signature: "pub fn band (prefs : & crate :: workspace :: Preferences , kind : LineKind) -> Option < [f32 ; 4] >"
---

Resolves the optional line background band color for diagnostic lines.

# Examples

```rust
use crawrad_project::scheme_preview::{band, LineKind};
use crawrad_project::workspace::Preferences;

let prefs = Preferences::default();
assert_eq!(band(&prefs, LineKind::Normal), None);
```
