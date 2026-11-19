---
id: acp-status-statustracker-finish
kind: doc
target_file: src/acp_status.rs
line_hint: 120
author: 
created: 
title: "StatusTracker::finish"
tags: []
signature: "pub fn finish (& mut self) -> StatusLine"
---

Ends the turn and dims the status line.

Step 5: the turn ended. The line stops and dims.

# Examples

```rust
use crawrad_agent::acp_status::StatusTracker;

let mut tracker = StatusTracker::new("claude", false);
let line = tracker.finish();
assert!(line.done);
```
