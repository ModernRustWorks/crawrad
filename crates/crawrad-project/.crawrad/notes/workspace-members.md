---
id: workspace-members
kind: doc
target_file: src/workspace.rs
line_hint: 127
author: 
created: 
title: "members"
tags: []
signature: "pub fn members (root : & Path) -> Vec < PathBuf >"
---

The member folders of an open root, as **absolute** paths.

`default-members` comes first: it is the *active project* of the group, with the
name Cargo already uses, and it is what decides which form the IDE opens
when the group has more than one.

**A standalone package counts as a single member: itself.** This is not a hidden
special case — it is what `cargo` sees when pointing to a folder with `[package]`
and without `[workspace]`, and refusing that folder would make the IDE say
"no project here" in front of a perfectly valid Rust project.

# Examples

```rust
use std::path::Path;
use crawrad_project::workspace::members;

let list = members(Path::new("non_existent_root_xyz"));
assert!(list.is_empty());
```
