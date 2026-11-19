---
id: diff-review-diffreviewmanager-any-pending
kind: doc
target_file: src/diff_review.rs
line_hint: 196
author: 
created: 
title: "DiffReviewManager::any_pending"
tags: []
signature: "pub fn any_pending (& self) -> Option < & DiffReview >"
---

Returns any pending review (for the editor bar or the global save check).

# Examples

```rust
use crawrad_agent::DiffReviewManager;

let manager = DiffReviewManager::new();
assert!(manager.any_pending().is_none());
```
