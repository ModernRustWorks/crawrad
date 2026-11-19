---
id: diff-review-diffreviewmanager-accept
kind: doc
target_file: src/diff_review.rs
line_hint: 231
author: 
created: 
title: "DiffReviewManager::accept"
tags: []
signature: "pub fn accept (& mut self , path : & Path) -> Result < String , String >"
---

Accepts every change of the file's pending review.

Accepts every change of the file's pending review.
Returns the proposed content to be written to the buffer with undo.

# Errors

Returns an error if there is no pending review or the review is not pending.

# Examples

```rust
use crawrad_agent::DiffReviewManager;
use std::path::{Path, PathBuf};

let mut manager = DiffReviewManager::new();
manager.propose_edit(PathBuf::from("main.rs"), "a".into(), "b".into(), "tool").unwrap();
let content = manager.accept(Path::new("main.rs")).unwrap();
assert_eq!(content, "b");
```
