---
id: creation-creation-advance
kind: doc
target_file: src/creation.rs
line_hint: 83
author: 
created: 
title: "Creation::advance"
tags: []
signature: "pub fn advance (& mut self , step : CreationStep)"
---

A step arrived through the channel.

Method, and not a `match` inside `Ide::harvest_creation`, because the
count of `completed` is the only thing here that can go wrong — and when wrong
it displays "3 of 5" with the first step still running.

# Examples

```rust
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Instant;
use crawrad_project::creation::{Creation, CreationStep};
use crawrad_project::workspace::ProjectType;

let (_tx, rx) = channel();
let mut creation = Creation {
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
creation.advance(CreationStep::Plan(vec![]));
assert!(creation.plan.is_empty());
```
