---
id: acp-status-statusfeed
kind: doc
target_file: src/acp_status.rs
line_hint: 215
author: 
created: 
title: "StatusFeed"
tags: []
signature: "pub struct StatusFeed"
---

The worker's side: a tracker that sends its line whenever it changes.

# Examples

```rust
use crawrad_agent::acp_status::StatusFeed;
use std::sync::mpsc::channel;

let (tx, _rx) = channel();
let feed = StatusFeed::start("claude", false, tx);
```
