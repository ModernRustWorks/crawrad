---
id: diff-review-diffreviewmode
kind: doc
target_file: src/diff_review.rs
line_hint: 17
author: 
created: 
title: "DiffReviewMode"
tags: []
signature: "pub enum DiffReviewMode"
---

User-configurable mode for reviewing AI-generated diffs.

# Examples

```rust
use crawrad_agent::diff_review::DiffReviewMode;

assert_eq!(DiffReviewMode::default(), DiffReviewMode::WaitForApproval);
```
