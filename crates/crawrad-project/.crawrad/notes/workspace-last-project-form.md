---
id: workspace-last-project-form
kind: doc
target_file: src/workspace.rs
line_hint: 318
author: 
created: 
title: "last_project_form"
tags: []
signature: "pub fn last_project_form (p : & Preferences , recent : & [PathBuf] , documents : impl Fn (& Path) -> Vec < PathBuf > ,) -> Option < PathBuf >"
---

Returns the form to open on startup based on preferences and recents.

The form to open on start (#555): the first form of the most recent
project, unless the user asked for the welcome instead
(`welcome_on_startup`, the "Reopen the last project on start" box).

Pure over its inputs, so the three refusals — box off, no recent, a
recent with no form — are proven without a disk.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::workspace::{last_project_form, Preferences};

let mut p = Preferences::default();
p.welcome_on_startup = true;
assert_eq!(last_project_form(&p, &[], |_| vec![]), None);
```
