---
id: diff-review-hunkstatus
kind: doc
target_file: src/diff_review.rs
line_hint: 56
author: 
created: 
title: "HunkStatus"
tags: []
signature: "pub enum HunkStatus"
---

State of a single change block (hunk).

# Examples

```rust
use crawrad_agent::HunkStatus;

assert_eq!(HunkStatus::default(), HunkStatus::Pending);
```
