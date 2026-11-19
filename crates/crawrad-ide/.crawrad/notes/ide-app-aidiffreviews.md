---
id: ide-app-aidiffreviews
kind: doc
target_file: src/ide_app.rs
line_hint: 369
author: 
created: 
title: "AiDiffReviews"
tags: []
signature: "pub type AiDiffReviews = crawrad_agent :: diff_review :: DiffReviewManager"
---

Manager for code diff reviews proposed by agents.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mgr = AiDiffReviews::default();
assert!(mgr.is_empty());
```
