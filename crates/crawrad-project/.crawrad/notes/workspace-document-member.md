---
id: workspace-document-member
kind: doc
target_file: src/workspace.rs
line_hint: 158
author: 
created: 
title: "document_member"
tags: []
signature: "pub fn document_member (document : & Path) -> Option < PathBuf >"
---

Returns which member owns this document.

Which member owns this document?

The document lives in `<member>/src/<name>.crawform`,
so the member is the grandparent of the file. Derived from the path,
and not stored in a field.

# Examples

```rust
use std::path::{Path, PathBuf};
use crawrad_project::workspace::document_member;

assert_eq!(
    document_member(Path::new("app/src/main.crawform")),
    Some(PathBuf::from("app"))
);
```
