---
id: workspace-projecttype-label
kind: doc
target_file: src/workspace.rs
line_hint: 1362
author: 
created: 
title: "ProjectType::label"
tags: []
signature: "pub fn label (self) -> & 'static str"
---

The UI label, in English like the entire interface.

# Examples

```rust
use crawrad_project::workspace::ProjectType;

assert_eq!(ProjectType::Gui.label(), "GUI Application");
```
