---
id: review-tool-tool-name
kind: doc
target_file: src/review_tool.rs
line_hint: 74
author:
created:
title: "tool_name"
tags: []
signature: "pub fn tool_name (accept : bool) -> & 'static str"
---

The tool name the audit line is written under, by the decision taken.

# Examples

```rust
use crawrad_mcp::review_tool;

assert_eq!(review_tool::tool_name(true), "review.accept");
assert_eq!(review_tool::tool_name(false), "review.reject");
```
