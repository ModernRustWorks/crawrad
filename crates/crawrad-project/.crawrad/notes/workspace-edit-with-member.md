---
id: workspace-edit-with-member
kind: doc
target_file: src/workspace_edit.rs
line_hint: 55
author: 
created: 
title: "with_member"
tags: []
signature: "pub fn with_member (manifest : & str , member : & str) -> Result < String , String >"
---

Appends a member to `[workspace] members` in manifest text.

`manifest` with `member` appended to `[workspace] members` (created when the
key is missing). A member already listed is not listed twice.

# Examples

```rust
use crawrad_project::workspace_edit::with_member;

let manifest = "[workspace]\nmembers = []\n";
let updated = with_member(manifest, "crates/app").unwrap();
assert!(updated.contains("\"crates/app\""));
```

# Errors

Returns an error if `manifest` has no `[workspace]` section.
