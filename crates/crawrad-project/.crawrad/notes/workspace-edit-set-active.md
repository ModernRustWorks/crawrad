---
id: workspace-edit-set-active
kind: doc
target_file: src/workspace_edit.rs
line_hint: 151
author: 
created: 
title: "set_active"
tags: []
signature: "pub fn set_active (root : & Path , member : & Path) -> Result < () , String >"
---

Makes `member` the workspace's active project.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::workspace_edit::set_active;

// Avoid writing files during doctests.
let _ = set_active(Path::new("."), Path::new("subcrate"));
```

# Errors

Returns an error if `member` is outside `root` or file IO fails.
