---
id: session-start-timeout
kind: doc
target_file: src/session.rs
line_hint: 28
author: 
created: 
title: "START_TIMEOUT"
tags: []
signature: "pub const START_TIMEOUT : Duration = Duration :: from_secs (60)"
---

How long starting the program may take (building is not part of it).

# Examples

```rust
use crawrad_dap::session::START_TIMEOUT;

assert!(START_TIMEOUT.as_secs() > 0);
```
