---
id: acp-status-statusfeed-on-event
kind: doc
target_file: src/acp_status.rs
line_hint: 237
author: 
created: 
title: "StatusFeed::on_event"
tags: []
signature: "pub fn on_event (& mut self , event : & AcpStreamEvent)"
---

Folds one event, sending the line when it changed.

# Examples

```rust
use crawrad_agent::AcpStreamEvent;
use crawrad_agent::acp_status::StatusFeed;
use std::sync::mpsc::channel;

let (tx, _rx) = channel();
let mut feed = StatusFeed::start("claude", false, tx);
feed.on_event(&AcpStreamEvent::ThoughtDelta("idea".to_string()));
```
