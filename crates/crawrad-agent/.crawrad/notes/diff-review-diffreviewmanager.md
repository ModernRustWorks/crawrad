---
id: diff-review-diffreviewmanager
kind: doc
target_file: src/diff_review.rs
line_hint: 172
author: 
created: 
title: "DiffReviewManager"
tags: []
signature: "pub struct DiffReviewManager"
---

Thread-safe manager of the active diff reviews.

# Examples

```rust
use crawrad_agent::DiffReviewManager;

let manager = DiffReviewManager::new();
assert!(manager.any_pending().is_none());
```
