---
id: diff-review-diffreviewmanager-clear
kind: doc
target_file: src/diff_review.rs
line_hint: 275
author: 
created: 
title: "DiffReviewManager::clear"
tags: []
signature: "pub fn clear (& mut self)"
---

Clears every pending review.

# Examples

```rust
use crawrad_agent::DiffReviewManager;

let mut manager = DiffReviewManager::new();
manager.clear();
assert!(manager.any_pending().is_none());
```
