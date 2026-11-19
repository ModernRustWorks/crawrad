---
id: acp-status-statuswatch
kind: doc
target_file: src/acp_status.rs
line_hint: 261
author: 
created: 
title: "StatusWatch"
tags: []
signature: "pub struct StatusWatch"
---

The shell's side: the latest line received, ticked to the current second.

# Examples

```rust
use crawrad_agent::acp_status::StatusWatch;

let (_tx, mut watch) = StatusWatch::channel("claude");
let line = watch.current();
assert!(!line.done);
```
