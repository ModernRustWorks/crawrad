---
id: creation-createdproject
kind: doc
target_file: src/creation.rs
line_hint: 251
author: 
created: 
title: "CreatedProject"
tags: []
signature: "pub struct CreatedProject"
---

The newly created project, and what went wrong without preventing creation.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::creation::CreatedProject;

let project = CreatedProject {
    root: PathBuf::from("."),
    warning: None,
};
assert!(project.warning.is_none());
```
