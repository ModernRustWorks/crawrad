---
id: diff-review-diffreviewmanager-get-pending-review
kind: doc
target_file: src/diff_review.rs
line_hint: 190
author: 
created: 
title: "DiffReviewManager::get_pending_review"
tags: []
signature: "pub fn get_pending_review (& self , path : & Path) -> Option < & DiffReview >"
---

Returns a reference to the file's pending review.

# Examples

```rust
use crawrad_agent::DiffReviewManager;
use std::path::Path;

let manager = DiffReviewManager::new();
assert!(manager.get_pending_review(Path::new("src/main.rs")).is_none());
```
