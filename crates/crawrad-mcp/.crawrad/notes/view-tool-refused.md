---
id: view-tool-refused
kind: doc
target_file: src/view_tool.rs
line_hint: 1
author: 
created: 
title: "refused"
tags: []
signature: "pub fn refused (id : & str , why : & str) -> Answer"
---

The answer when the IDE would not run the action now.

An agent outside the IDE has no greyed menu to look at: before this answer,
`ide.run_action` said *"executed"* for an action the IDE had refused, and the
agent went on as if it had happened. The reason is the IDE's own sentence,
the one the status bar shows.

# Examples

```rust
use crawrad_mcp::view_tool::refused;

let (result, status) = refused("project.generate", "Generate needs a form in the project");
assert_eq!(status, "error");
assert!(serde_json::to_string(&result).unwrap().contains("action-disabled"));
```
