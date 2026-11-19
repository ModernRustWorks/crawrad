---
id: workspace-edit-add-member
kind: doc
target_file: src/workspace_edit.rs
line_hint: 139
author: 
created: 
title: "add_member"
tags: []
signature: "pub fn add_member (root : & Path , member : & Path) -> Result < () , String >"
---

Adds the package folder `member` to the workspace at `root`.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::workspace_edit::add_member;

// Avoid writing files during doctests.
let _ = add_member(Path::new("."), Path::new("subcrate"));
```

# Errors

Returns an error if `member` has no `Cargo.toml`, is outside `root`, or file IO fails.
