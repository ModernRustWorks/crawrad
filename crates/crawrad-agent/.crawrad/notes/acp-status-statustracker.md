---
id: acp-status-statustracker
kind: doc
target_file: src/acp_status.rs
line_hint: 39
author: 
created: 
title: "StatusTracker"
tags: []
signature: "pub struct StatusTracker"
---

Folds a turn's events into its status line.

# Examples

```rust
use crawrad_agent::acp_status::StatusTracker;

let tracker = StatusTracker::new("claude", false);
let line = tracker.current();
assert!(!line.done);
```
