---
id: workspace-member-documents
kind: doc
target_file: src/workspace.rs
line_hint: 183
author: 
created: 
title: "member_documents"
tags: []
signature: "pub fn member_documents (member : & Path) -> Vec < PathBuf >"
---

The drawings of one member in absolute paths and in order.

The drawings of **one** member: `<member>/src/*.crawform`,
without descending, in absolute paths and in order.

# Examples

```rust
use std::path::Path;
use crawrad_project::workspace::member_documents;

let docs = member_documents(Path::new("non_existent_member_xyz"));
assert!(docs.is_empty());
```
