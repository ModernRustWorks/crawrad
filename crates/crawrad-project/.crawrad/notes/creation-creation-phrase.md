---
id: creation-creation-phrase
kind: doc
target_file: src/creation.rs
line_hint: 100
author: 
created: 
title: "Creation::phrase"
tags: []
signature: "pub fn phrase (& self) -> String"
---

Returns the current progress status in one line.

Current progress status in one line. **English**, as the whole UI.

Pure enough to be measured without a window, and that is why it exists
separately from rendering: writing logic inside rendering was how the
ruler was left without guards.

The empty plan is the millisecond gap between `spawn` and the first
`CreationStep::Plan`. `starting…` is honest there; an invented `step 1 of 1`
would be the UI asserting a total it does not yet know.

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
assert_eq!(creation.phrase(), "starting…");
```
