---
id: workspace-step-label
kind: doc
target_file: src/workspace.rs
line_hint: 1422
author: 
created: 
title: "Step::label"
tags: []
signature: "pub fn label (self , project_type : ProjectType) -> String"
---

What the step displays on screen, in English like the whole UI.

**Names the cargo command**, and this is ADR 0018 at work: the IDE talks
to cargo instead of reimplementing it, and whoever sees `cargo add eframe@0.36`
on screen knows what to look for when the step fails without network.

`project_type` enters because the dependency belongs to it — writing `eframe@0.36`
here would be a second copy of a value already in [`ProjectType::dependency`].

# Examples

```rust
use crawrad_project::workspace::{Step, ProjectType};

assert!(Step::Package.label(ProjectType::Gui).contains("cargo new"));
```
