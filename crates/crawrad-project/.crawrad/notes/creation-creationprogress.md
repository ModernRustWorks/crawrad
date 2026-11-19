---
id: creation-creationprogress
kind: doc
target_file: src/creation.rs
line_hint: 42
author: 
created: 
title: "CreationProgress"
tags: []
signature: "pub enum CreationProgress"
---

What arrives at the shell through the creation channel.

# Examples

```rust
use crawrad_project::creation::{CreationProgress, CreationStep};

let prog = CreationProgress::Running(CreationStep::Plan(vec![]));
match prog {
    CreationProgress::Running(_) => assert!(true),
    _ => unreachable!(),
}
```
