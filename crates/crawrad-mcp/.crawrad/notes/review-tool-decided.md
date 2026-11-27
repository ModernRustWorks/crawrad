---
id: review-tool-decided
kind: doc
target_file: src/review_tool.rs
line_hint: 58
author:
created:
title: "decided"
tags: []
signature: "pub fn decided (path : & str , accept : bool) -> ToolCallResult"
---

The decision was applied.

`accept` picks the word the agent reads back, and that same word goes in the JSON, for whoever reads the answer by program.

# Examples

```rust
use crawrad_mcp::review_tool;

let accepted = review_tool::decided("src/main.rs", true);
assert!(!accepted.is_error);
let rejected = review_tool::decided("src/main.rs", false);
assert!(!rejected.is_error);
```
