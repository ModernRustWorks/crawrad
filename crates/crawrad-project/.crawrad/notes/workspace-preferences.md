---
id: workspace-preferences
kind: doc
target_file: src/workspace.rs
line_hint: 947
author: 
created: 
title: "Preferences"
tags: []
signature: "pub struct Preferences"
---

Global IDE preferences not tied to a single project.

# Examples

```rust
use crawrad_project::workspace::Preferences;

let p = Preferences::default();
assert_eq!(p.editor_font_size, 11);
```
