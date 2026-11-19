---
id: acp-status-statuswatch-poll
kind: doc
target_file: src/acp_status.rs
line_hint: 280
author: 
created: 
title: "StatusWatch::poll"
tags: []
signature: "pub fn poll (& mut self) -> StatusLine"
---

Takes every line received so far and returns the current one.

# Examples

```rust
use crawrad_agent::acp_status::StatusWatch;

let (_tx, mut watch) = StatusWatch::channel("claude");
let line = watch.poll();
assert!(!line.done);
```
