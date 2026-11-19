---
id: workspace-targets-timestamp
kind: doc
target_file: src/workspace.rs
line_hint: 1508
author: 
created: 
title: "targets_timestamp"
tags: []
signature: "pub fn targets_timestamp (root : & Path , member : Option < & Path >) -> Option < std :: time :: SystemTime >"
---

The disk stamp that makes [`group_executables`]'s answer valid.

**Three `stat` calls, and these three because they are the three places where
an executable target can arise**: the group manifest (a member entered or left),
the member manifest (gained a `[[bin]]`), and its `src/` folder (a `main.rs`
appeared, which cargo discovers automatically). The NEWEST of the three serves
as the stamp; any of them changing changes the set, and the query is rerun.

Compared with `!=` and not `>`: a file clock can move backwards — restoring
a backup, switching branches — and what matters is *changed*, not *newer*.

Three `stat` calls per frame is the price, and it is orders of magnitude less
than what Explorer already pays: that panel runs `read_dir` on each member's
`src/` on every frame it is open.

# Examples

```rust
use std::path::Path;
use crawrad_project::workspace::targets_timestamp;

let stamp = targets_timestamp(Path::new("."), None);
```
