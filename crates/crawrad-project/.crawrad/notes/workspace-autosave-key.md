---
id: workspace-autosave-key
kind: doc
target_file: src/workspace.rs
line_hint: 510
author: 
created: 
title: "AutoSave::key"
tags: []
signature: "pub fn key (self) -> & 'static str"
---

The value in `preferences.txt`.

# Examples

```rust
use crawrad_project::workspace::AutoSave;

assert_eq!(AutoSave::Off.key(), "off");
```
