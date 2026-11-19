---
id: workspace-autosave-position
kind: doc
target_file: src/workspace.rs
line_hint: 505
author: 
created: 
title: "AutoSave::position"
tags: []
signature: "pub fn position (self) -> usize"
---

Its place in [`AutoSave::ALL`], the drop-down's item.

# Examples

```rust
use crawrad_project::workspace::AutoSave;

assert_eq!(AutoSave::Off.position(), 0);
```
