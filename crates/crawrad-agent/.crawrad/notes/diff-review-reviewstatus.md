---
id: diff-review-reviewstatus
kind: doc
target_file: src/diff_review.rs
line_hint: 88
author: 
created: 
title: "ReviewStatus"
tags: []
signature: "pub enum ReviewStatus"
---

Overall state of a diff review.

# Examples

```rust
use crawrad_agent::ReviewStatus;

assert_eq!(ReviewStatus::default(), ReviewStatus::Pending);
```
