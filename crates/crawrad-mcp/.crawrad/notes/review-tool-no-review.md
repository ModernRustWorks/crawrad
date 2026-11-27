---
id: review-tool-no-review
kind: doc
target_file: src/review_tool.rs
line_hint: 47
author:
created:
title: "no_review"
tags: []
signature: "pub fn no_review (path : Option < & str >) -> ToolCallResult"
---

Asked to decide, and there is nothing to decide.

`path` is absent when no file was under review at all, and present when that particular file has none.

# Examples

```rust
use crawrad_mcp::review_tool;

// Nothing under review at all.
assert!(review_tool::no_review(None).is_error);
// That one file has none.
assert!(review_tool::no_review(Some("src/main.rs")).is_error);
```
