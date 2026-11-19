---
id: creation-creation
kind: doc
target_file: src/creation.rs
line_hint: 49
author: 
created: 
title: "Creation"
tags: []
signature: "pub struct Creation"
---

The in-flight creation, and what the dialog displays about it.

# Examples

```rust
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Instant;
use crawrad_project::creation::Creation;
use crawrad_project::workspace::ProjectType;

let (_tx, rx) = channel();
let creation = Creation {
    rx,
    project_type: ProjectType::Gui,
    root: PathBuf::from("."),
    plan: Vec::new(),
    completed: 0,
    current: None,
    since: Instant::now(),
    outcome: None,
    show: true,
};
assert!(creation.show);
```
