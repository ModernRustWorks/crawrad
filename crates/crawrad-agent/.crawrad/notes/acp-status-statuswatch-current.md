---
id: acp-status-statuswatch-current
kind: doc
target_file: src/acp_status.rs
line_hint: 288
author: 
created: 
title: "StatusWatch::current"
tags: []
signature: "pub fn current (& self) -> StatusLine"
---

The latest line, its seconds counted to now.

# Examples

```rust
use crawrad_agent::acp_status::StatusWatch;

let (_tx, watch) = StatusWatch::channel("claude");
let line = watch.current();
assert!(!line.done);
```
