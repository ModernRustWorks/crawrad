---
id: workspace-autosave-label
kind: doc
target_file: src/workspace.rs
line_hint: 490
author: 
created: 
title: "AutoSave::label"
tags: []
signature: "pub fn label (self) -> & 'static str"
---

The words the drop-down shows.

# Examples

```rust
use crawrad_project::workspace::AutoSave;

assert_eq!(AutoSave::Off.label(), "Off");
```
