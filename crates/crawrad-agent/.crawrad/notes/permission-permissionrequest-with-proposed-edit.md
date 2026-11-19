---
id: permission-permissionrequest-with-proposed-edit
kind: doc
target_file: src/permission.rs
line_hint: 192
author: 
created: 
title: "PermissionRequest::with_proposed_edit"
tags: []
signature: "pub fn with_proposed_edit(mut self, edit: Option<crate::edit_ask::ProposedEdit>) -> Self"
---

Attaches the file change an edit tool asks for.

The IDE then shows the request as a diff instead of the permission card.

# Examples

```rust
use crawrad_agent::PermissionRequest;
use std::time::Duration;

let (tx, _rx) = std::sync::mpsc::channel();
let req = PermissionRequest::new(1, "s", "Bash", None, "", Vec::new(), tx, Duration::from_secs(9))
    .with_proposed_edit(None);
assert!(req.proposed_edit.is_none());
```
