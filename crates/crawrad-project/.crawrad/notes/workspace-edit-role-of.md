---
id: workspace-edit-role-of
kind: doc
target_file: src/workspace_edit.rs
line_hint: 166
author: 
created: 
title: "role_of"
tags: []
signature: "pub fn role_of (path : & Path) -> WorkspaceRole"
---

What a folder is to the workspace, for the Explorer's context menu.

# Examples

```rust
use std::path::Path;
use crawrad_actions::WorkspaceRole;
use crawrad_project::workspace_edit::role_of;

let role = role_of(Path::new("non_existent_folder_xyz"));
assert_eq!(role, WorkspaceRole::Other);
```
