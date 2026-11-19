---
id: run-tool-answer-2
kind: doc
target_file: src/run_tool.rs
line_hint: 28
author: 
created: 
title: "answer"
tags: []
signature: "pub fn answer (ran : Ran) -> Answer"
---

Converts a cargo execution result into a tool answer and audit status.

A cargo run as the tool answers it: a failed build still answers, as an
`error` in the audit; cargo that would not start is a `spawn-error`.

# Examples

```rust
use crawrad_mcp::run_tool::answer;
use serde_json::json;

let (result, status) = answer(Ok((json!({}), "done".to_string(), true)));
assert_eq!(status, "applied");
assert!(!result.is_error);
```
