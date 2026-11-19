---
id: workspace-edit-with-active
kind: doc
target_file: src/workspace_edit.rs
line_hint: 98
author: 
created: 
title: "with_active"
tags: []
signature: "pub fn with_active (manifest : & str , member : & str) -> Result < String , String >"
---

`manifest` with `member` as the ONLY `default-members` (the active project).

# Examples

```rust
use crawrad_project::workspace_edit::with_active;

let manifest = "[workspace]\nmembers = [\"app\"]\n";
let updated = with_active(manifest, "app").unwrap();
assert!(updated.contains("default-members = [\"app\"]"));
```

# Errors

Returns an error if `manifest` has no `[workspace]` section.
