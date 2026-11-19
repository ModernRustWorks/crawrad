---
id: acp-status-statustracker-at
kind: doc
target_file: src/acp_status.rs
line_hint: 132
author: 
created: 
title: "StatusTracker::at"
tags: []
signature: "pub fn at (& self , elapsed : Duration) -> StatusLine"
---

The line as it reads `elapsed` after the prompt was sent.

# Examples

```rust
use crawrad_agent::acp_status::StatusTracker;
use std::time::Duration;

let tracker = StatusTracker::new("claude", false);
let line = tracker.at(Duration::from_secs(5));
assert!(!line.done);
```
