---
id: workspace-recent
kind: doc
target_file: src/workspace.rs
line_hint: 401
author: 
created: 
title: "Recent"
tags: []
signature: "pub struct Recent"
---

What the welcome screen shows next to a recent folder.

The three fields exist because the previous list showed only **name and parent folder**
and nothing else: two `app` folders on different drives looked identical on screen,
and choosing between them became guesswork. None of them is new data — the folder
is the workspace (ADR 0017), counting `.crawform` is what [`documents_in`] already
does, and the date comes from disk.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::workspace::Recent;

let r = Recent {
    path: PathBuf::from("app"),
    forms: 0,
    modified: None,
};
assert_eq!(r.forms, 0);
```
