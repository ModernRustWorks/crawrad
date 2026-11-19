---
id: workspace-edit-role-of-item
kind: doc
target_file: src/workspace_edit.rs
line_hint: 157
author: 
created: 
title: "role_of_item"
tags: []
signature: "pub fn role_of_item (path : & Path , is_dir : bool) -> WorkspaceRole"
---

The role of an Explorer item: a file is always [`WorkspaceRole::Other`].

# Examples

```rust
use std::path::Path;
use crawrad_actions::WorkspaceRole;
use crawrad_project::workspace_edit::role_of_item;

assert_eq!(role_of_item(Path::new("file.rs"), false), WorkspaceRole::Other);
```
