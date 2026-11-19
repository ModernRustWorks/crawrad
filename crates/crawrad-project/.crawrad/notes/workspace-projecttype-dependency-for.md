---
id: workspace-projecttype-dependency-for
kind: doc
target_file: src/workspace.rs
line_hint: 1388
author: 
created: 
title: "ProjectType::dependency_for"
tags: []
signature: "pub fn dependency_for (self , target : crawrad_codegen :: AlvoGerador) -> Option < & 'static str >"
---

The dependency that `cargo add` adds according to the chosen target.

# Examples

```rust
use crawrad_codegen::AlvoGerador;
use crawrad_project::workspace::ProjectType;

assert_eq!(ProjectType::Gui.dependency_for(AlvoGerador::Egui), Some("eframe@0.36"));
assert_eq!(ProjectType::Console.dependency_for(AlvoGerador::Egui), None);
```
