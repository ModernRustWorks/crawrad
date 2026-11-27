---
id: review-tool-path-asked
kind: doc
target_file: src/review_tool.rs
line_hint: 82
author:
created:
title: "path_asked"
tags: []
signature: "pub fn path_asked (args : & Value) -> Option < std :: path :: PathBuf >"
---

The path the decision falls on when the agent names one.

The shell supplies the fallback; this keeps the reading of the argument in one place.

# Examples

```rust
use crawrad_mcp::review_tool;
use serde_json::json;

let asked = review_tool::path_asked(&json!({ "path": "src/main.rs" }));
assert_eq!(asked.unwrap().to_string_lossy(), "src/main.rs");

// No path named: the shell supplies the fallback.
assert!(review_tool::path_asked(&json!({})).is_none());
```
