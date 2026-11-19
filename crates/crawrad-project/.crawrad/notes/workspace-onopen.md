---
id: workspace-onopen
kind: doc
target_file: src/workspace.rs
line_hint: 211
author: 
created: 
title: "OnOpen"
tags: []
signature: "pub enum OnOpen"
---

What to do with the documents found in a root.

An `enum` and not an `Option<PathBuf>` plus a `Vec`: the three cases have
**different** screen responses — an empty folder invites creating a project,
choosing shows a list, and the case of one opens directly. With `Option` the middle
case would become an `if list.len() > 1` floating in the UI, which is where
it gets lost.

# Examples

```rust
use crawrad_project::workspace::OnOpen;

assert_eq!(OnOpen::NoForm, OnOpen::NoForm);
```
