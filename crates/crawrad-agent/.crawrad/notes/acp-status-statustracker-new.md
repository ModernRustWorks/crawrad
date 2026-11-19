---
id: acp-status-statustracker-new
kind: doc
target_file: src/acp_status.rs
line_hint: 52
author: 
created: 
title: "StatusTracker::new"
tags: []
signature: "pub fn new (agent : & str , via_cli : bool) -> Self"
---

Step 1: the prompt was just sent (through ACP, or through the CLI).

# Examples

```rust
use crawrad_agent::acp_status::StatusTracker;

let tracker = StatusTracker::new("claude", false);
let line = tracker.current();
assert!(line.line.contains("claude"));
```
