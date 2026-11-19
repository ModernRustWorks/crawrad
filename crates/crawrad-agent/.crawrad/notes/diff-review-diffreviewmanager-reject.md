---
id: diff-review-diffreviewmanager-reject
kind: doc
target_file: src/diff_review.rs
line_hint: 253
author: 
created: 
title: "DiffReviewManager::reject"
tags: []
signature: "pub fn reject (& mut self , path : & Path) -> Result < String , String >"
---

Rejects every change of the file's pending review.

Rejects every change of the file's pending review.
Returns the original content byte for byte, leaving no trace.

# Errors

Returns an error if there is no pending review or the review is not pending.

# Examples

```rust
use crawrad_agent::DiffReviewManager;
use std::path::{Path, PathBuf};

let mut manager = DiffReviewManager::new();
manager.propose_edit(PathBuf::from("main.rs"), "a".into(), "b".into(), "tool").unwrap();
let content = manager.reject(Path::new("main.rs")).unwrap();
assert_eq!(content, "a");
```
