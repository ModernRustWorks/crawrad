---
id: workspace-read-recent
kind: doc
target_file: src/workspace.rs
line_hint: 256
author: 
created: 
title: "read_recent"
tags: []
signature: "pub fn read_recent () -> Vec < PathBuf >"
---

One folder per line, and nothing more.

Not JSON by design. The content is a list of paths, and a format with schema
would invite hanging the rest of the IDE state onto it — which is state
**per folder** and belongs to the root's `.crawrad/`, not a global file.
Empty lines and duplicate lines are ignored on read, because this file is
editable by hand and someone will edit it by hand.

# Examples

```rust
use crawrad_project::workspace::read_recent;

let recent = read_recent();
assert!(recent.len() <= 10);
```
