---
id: workspace-projecttype-description
kind: doc
target_file: src/workspace.rs
line_hint: 1371
author: 
created: 
title: "ProjectType::description"
tags: []
signature: "pub fn description (self) -> & 'static str"
---

The line explaining what will happen, also in English.

# Examples

```rust
use crawrad_project::workspace::ProjectType;

assert!(ProjectType::Gui.description().contains("form"));
```
