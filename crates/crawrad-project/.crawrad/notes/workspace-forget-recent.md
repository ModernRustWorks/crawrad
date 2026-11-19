---
id: workspace-forget-recent
kind: doc
target_file: src/workspace.rs
line_hint: 330
author: 
created: 
title: "forget_recent"
tags: []
signature: "pub fn forget_recent (folder : & Path)"
---

Takes a folder out of the recent folders on disk — the list, never the folder.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::workspace::forget_recent;

// Avoid writing files during doctests.
forget_recent(Path::new("my_project"));
```
