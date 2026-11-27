---
id: review-tool-failed
kind: doc
target_file: src/review_tool.rs
line_hint: 68
author:
created:
title: "failed"
tags: []
signature: "pub fn failed (reason : impl std :: fmt :: Display) -> ToolCallResult"
---

The decision failed to reach the buffer.

# Examples

```rust
use crawrad_mcp::review_tool;

let answer = review_tool::failed("the buffer moved under the edit");
assert!(answer.is_error);
```
