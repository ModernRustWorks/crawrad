---
id: acp-status-statusfeed-finish
kind: doc
target_file: src/acp_status.rs
line_hint: 252
author: 
created: 
title: "StatusFeed::finish"
tags: []
signature: "pub fn finish (& mut self) -> StatusLine"
---

Ends the line (step 5), sends it, and returns it.

# Examples

```rust
use crawrad_agent::acp_status::StatusFeed;
use std::sync::mpsc::channel;

let (tx, _rx) = channel();
let mut feed = StatusFeed::start("claude", false, tx);
let line = feed.finish();
assert!(line.done);
```
