---
id: diff-review-diffreviewmanager-propose-edit
kind: doc
target_file: src/diff_review.rs
line_hint: 201
author: 
created: 
title: "DiffReviewManager::propose_edit"
tags: []
signature: "pub fn propose_edit (& mut self , path : PathBuf , original_content : String , proposed_content : String , author_tool : & str ,) -> Result < DiffReview , String >"
---

Registers an edit proposal coming from a tool.

Registers an edit proposal coming from a tool.
Returns an error if the file already has an active pending review.

# Errors

Returns an error if the file already has an active pending diff review.

# Examples

```rust
use crawrad_agent::DiffReviewManager;
use std::path::PathBuf;

let mut manager = DiffReviewManager::new();
let res = manager.propose_edit(PathBuf::from("main.rs"), "a".into(), "b".into(), "tool");
assert!(res.is_ok());
```
