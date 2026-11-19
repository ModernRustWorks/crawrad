---
id: workspace-describe-recent
kind: doc
target_file: src/workspace.rs
line_hint: 421
author: 
created: 
title: "describe_recent"
tags: []
signature: "pub fn describe_recent (folder : & Path) -> Recent"
---

Reads from disk what is known about a recent folder.

Touches the disk, and therefore the result is cached by the caller instead of
recomputed on each frame: up to `RECENT_LIMIT` directory scans, and at 60 frames
per second that would be the welcome screen hammering the disk with nothing changing.

# Examples

```rust
use std::path::Path;
use crawrad_project::workspace::describe_recent;

let r = describe_recent(Path::new("non_existent_folder_xyz"));
assert_eq!(r.forms, 0);
```
