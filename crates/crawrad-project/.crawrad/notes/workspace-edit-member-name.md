---
id: workspace-edit-member-name
kind: doc
target_file: src/workspace_edit.rs
line_hint: 116
author: 
created: 
title: "member_name"
tags: []
signature: "pub fn member_name (root : & Path , member : & Path) -> Result < String , String >"
---

The member's name as the root lists it.

The member's name as the root lists it: its path relative to the root,
with `/`. A folder outside the workspace's folder is refused — cargo would
need the member to point back at the root, and that writes to the user's
own `Cargo.toml`, which this never does on its own.

# Examples

```rust
use std::path::Path;
use crawrad_project::workspace_edit::member_name;

let root = Path::new("/workspace");
let member = Path::new("/workspace/crates/app");
assert_eq!(member_name(root, member).unwrap(), "crates/app");
```

# Errors

Returns an error if `member` is outside `root` or identical to `root`.
