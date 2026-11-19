---
id: edit-activefileparams
kind: doc
target_file: src/edit.rs
line_hint: 87
author: 
created: 
title: "ActiveFileParams"
tags: []
signature: "pub struct ActiveFileParams < 'a >"
---

Plain values for `editor.get_active_file`.

# Examples

```rust
use crawrad_mcp::edit::ActiveFileParams;

let params = ActiveFileParams {
    path: Some("src/lib.rs"),
    is_dirty: false,
    is_editable: true,
    center_view: "code",
    tab_file: Some("src/lib.rs"),
    read_only: false,
    caret: Some((1, 0)),
    status: None,
    breadcrumb: Vec::new(),
    disk_changed: false,
    deleted_on_disk: false,
};
assert_eq!(params.path, Some("src/lib.rs"));
```
