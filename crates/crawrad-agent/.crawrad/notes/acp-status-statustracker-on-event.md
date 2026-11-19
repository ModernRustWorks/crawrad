---
id: acp-status-statustracker-on-event
kind: doc
target_file: src/acp_status.rs
line_hint: 65
author: 
created: 
title: "StatusTracker::on_event"
tags: []
signature: "pub fn on_event (& mut self , event : & AcpStreamEvent) -> bool"
---

Folds one stream event into the status line.

Folds one event. Returns whether the line changed.

# Examples

```rust
use crawrad_agent::AcpStreamEvent;
use crawrad_agent::acp_status::StatusTracker;

let mut tracker = StatusTracker::new("claude", false);
let changed = tracker.on_event(&AcpStreamEvent::ThoughtDelta("thinking".to_string()));
assert!(changed);
```
