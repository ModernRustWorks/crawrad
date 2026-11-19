---
id: workspace-remember-recent
kind: doc
target_file: src/workspace.rs
line_hint: 313
author: 
created: 
title: "remember_recent"
tags: []
signature: "pub fn remember_recent (folder : & Path)"
---

Puts the project just opened at the top of the recent folders on disk.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::workspace::remember_recent;

// Avoid writing files during doctests.
remember_recent(Path::new("my_project"));
```
