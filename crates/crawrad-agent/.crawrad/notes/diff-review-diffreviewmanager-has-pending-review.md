---
id: diff-review-diffreviewmanager-has-pending-review
kind: doc
target_file: src/diff_review.rs
line_hint: 184
author: 
created: 
title: "DiffReviewManager::has_pending_review"
tags: []
signature: "pub fn has_pending_review (& self , path : & Path) -> bool"
---

Whether there is a pending review for the given file.

# Examples

```rust
use crawrad_agent::DiffReviewManager;
use std::path::Path;

let manager = DiffReviewManager::new();
assert!(!manager.has_pending_review(Path::new("src/main.rs")));
```
