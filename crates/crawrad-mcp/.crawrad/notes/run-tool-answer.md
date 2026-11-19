---
id: run-tool-answer
kind: doc
target_file: src/run_tool.rs
line_hint: 14
author: 
created: 
title: "Answer"
tags: []
signature: "pub type Answer = (ToolCallResult , & 'static str)"
---

A tool's answer and the outcome its audit line records.

# Examples

```rust
use crawrad_mcp::run_tool::Answer;
use crawrad_mcp::protocol::ToolCallResult;

let ans: Answer = (ToolCallResult::success_text("ok"), "applied");
assert_eq!(ans.1, "applied");
```
