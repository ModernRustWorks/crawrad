---
id: explorer-workspacerole
kind: doc
target_file: src/explorer.rs
line_hint: 86
author: 
created: 
title: "WorkspaceRole"
tags: []
signature: "pub enum WorkspaceRole"
---

The role of a folder in an open Cargo workspace.

What a folder is to the open Cargo workspace (#471), so its context menu can
offer *Add Project…* on the root and *Set as Active Project* on a member.
Decided by `crawrad_project::workspace_edit::role_of`, which reads the disk;
the catalogue never does.

# Examples

```rust
use crawrad_actions::explorer::WorkspaceRole;

let role = WorkspaceRole::default();
assert_eq!(role, WorkspaceRole::Other);
```
