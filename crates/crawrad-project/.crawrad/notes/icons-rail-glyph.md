---
id: icons-rail-glyph
kind: doc
target_file: src/icons.rs
line_hint: 64
author: 
created: 
title: "rail_glyph"
tags: []
signature: "pub fn rail_glyph (panel : crawrad_actions :: Panel) -> & 'static str"
---

The glyph the rail button draws, or `""` for a panel with no rail entry.

# Examples

```rust
use crawrad_actions::Panel;
use crawrad_project::icons::rail_glyph;

assert!(!rail_glyph(Panel::Explorer).is_empty());
```
