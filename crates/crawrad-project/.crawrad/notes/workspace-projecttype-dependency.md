---
id: workspace-projecttype-dependency
kind: doc
target_file: src/workspace.rs
line_hint: 1397
author: 
created: 
title: "ProjectType::dependency"
tags: []
signature: "pub fn dependency (self) -> Option < & 'static str >"
---

The default dependency for `cargo add`.

# Examples

```rust
use crawrad_project::workspace::ProjectType;

assert_eq!(ProjectType::Gui.dependency(), Some("eframe@0.36"));
```
