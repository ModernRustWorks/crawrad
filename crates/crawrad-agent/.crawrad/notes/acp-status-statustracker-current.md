---
id: acp-status-statustracker-current
kind: doc
target_file: src/acp_status.rs
line_hint: 127
author: 
created: 
title: "StatusTracker::current"
tags: []
signature: "pub fn current (& self) -> StatusLine"
---

The line now (the seconds counted to this instant).

# Examples

```rust
use crawrad_agent::acp_status::StatusTracker;

let tracker = StatusTracker::new("claude", false);
let line = tracker.current();
assert!(!line.done);
```
