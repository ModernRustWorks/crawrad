---
id: workspace-prune
kind: doc
target_file: src/workspace.rs
line_hint: 390
author: 
created: 
title: "prune"
tags: []
signature: "pub fn prune (list : & [PathBuf] , exists : impl Fn (& Path) -> bool) -> Vec < PathBuf >"
---

Removes from the list what no longer exists.

The predicate enters as a parameter instead of the function calling `is_dir()`:
this allows proving pruning without creating and deleting real folders, and is the
same reason why the rest of this module is pure. Deleted folders, removed flash drives,
and moved projects are the normal case here, not the exception — a recents menu
full of dead lines is a menu people stop clicking.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::workspace::prune;

let list = vec![PathBuf::from("keep"), PathBuf::from("drop")];
let kept = prune(&list, |p| p.to_str() == Some("keep"));
assert_eq!(kept, vec![PathBuf::from("keep")]);
```
