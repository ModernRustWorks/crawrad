---
id: diff-review-diffreviewmanager-decide-change
kind: doc
target_file: src/diff_review.rs
line_hint: 276
author: 
created: 
title: "DiffReviewManager::decide_change"
tags: []
signature: "pub fn decide_change (& mut self , path : & Path , k : usize , accept : bool) -> Result < bool , String >"
---

Accepts or rejects one change of the file's pending review.

Accepting change `k` applies it to the original; rejecting puts it back in the
proposed text, so the change leaves the diff either way. Answers whether no
change is left: the caller then accepts the review whole (`base_content` keeps
the file as it was, for undo).

# Errors

Returns an error if there is no pending review or the review is not pending.

# Examples

```rust
use crawrad_agent::DiffReviewManager;
use std::path::{Path, PathBuf};

let mut manager = DiffReviewManager::new();
manager
    .propose_edit(PathBuf::from("main.rs"), "a\nb\n".into(), "A\nB\n".into(), "tool")
    .unwrap();
assert!(manager.decide_change(Path::new("main.rs"), 0, false).unwrap());
assert_eq!(manager.accept(Path::new("main.rs")).unwrap(), "a\nb\n");
```
