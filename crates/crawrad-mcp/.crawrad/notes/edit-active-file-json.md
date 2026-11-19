---
id: edit-active-file-json
kind: doc
target_file: src/edit.rs
line_hint: 102
author: 
created: 
title: "active_file_json"
tags: []
signature: "pub fn active_file_json (params : ActiveFileParams < '_ >) -> (Value , String)"
---

The JSON answer and message for `editor.get_active_file`.

# Examples

```rust
use crawrad_mcp::edit::{active_file_json, ActiveFileParams};

let params = ActiveFileParams {
    path: None,
    is_dirty: false,
    is_editable: true,
    center_view: "code",
    tab_file: None,
    read_only: false,
    caret: None,
    status: None,
    breadcrumb: Vec::new(),
    disk_changed: false,
    deleted_on_disk: false,
};
let (json, msg) = active_file_json(params);
assert!(json["path"].is_null());
assert!(msg.contains("No file"));
```
