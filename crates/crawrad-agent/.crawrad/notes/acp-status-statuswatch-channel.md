---
id: acp-status-statuswatch-channel
kind: doc
target_file: src/acp_status.rs
line_hint: 269
author: 
created: 
title: "StatusWatch::channel"
tags: []
signature: "pub fn channel (agent : & str) -> (Sender < StatusLine > , Self)"
---

Creates a channel pair for status tracking across a turn.

A channel for one turn: the sender goes to the worker; the watch starts
on step 1 so the card has its line before the worker's first message.

# Examples

```rust
use crawrad_agent::acp_status::StatusWatch;

let (_tx, mut watch) = StatusWatch::channel("claude");
let line = watch.poll();
assert!(!line.done);
```
