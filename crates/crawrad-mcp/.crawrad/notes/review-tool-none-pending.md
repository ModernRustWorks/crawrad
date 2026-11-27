---
id: review-tool-none-pending
kind: doc
target_file: src/review_tool.rs
line_hint: 16
author:
created:
title: "none_pending"
tags: []
signature: "pub fn none_pending () -> ToolCallResult"
---

What `review.status` answers when no diff review is waiting.

Not an error: asking and hearing "nothing" is ordinary use of the tool.

# Examples

```rust
use crawrad_mcp::review_tool;

let answer = review_tool::none_pending();
assert!(!answer.is_error);
```
