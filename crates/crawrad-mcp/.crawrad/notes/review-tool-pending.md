---
id: review-tool-pending
kind: doc
target_file: src/review_tool.rs
line_hint: 22
author:
created:
title: "pending"
tags: []
signature: "pub fn pending (path : & str , author_tool : & str , additions : usize , deletions : usize , changes : usize) -> ToolCallResult"
---

The waiting review, counted: who proposed it, on which file, and how big.

The shell reads those numbers off the review it holds; this only shapes the answer.

# Examples

```rust
use crawrad_mcp::review_tool;

let answer = review_tool::pending("src/main.rs", "claude", 12, 3, 2);
assert!(!answer.is_error);
```
