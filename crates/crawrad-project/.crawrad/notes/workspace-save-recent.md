---
id: workspace-save-recent
kind: doc
target_file: src/workspace.rs
line_hint: 267
author: 
created: 
title: "save_recent"
tags: []
signature: "pub fn save_recent (list : & [PathBuf])"
---

Saves the list of recent folders to disk.

Saves the list. Fails silently: not being able to remember the last folder is
an annoyance, and interrupting what the user was doing to announce it would
be worse than the problem.

# Examples

```rust,no_run
use std::path::PathBuf;
use crawrad_project::workspace::save_recent;

// Avoid writing files during doctests.
save_recent(&[PathBuf::from("my_proj")]);
```
