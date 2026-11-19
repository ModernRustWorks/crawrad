---
id: acp-status-with-elapsed
kind: doc
target_file: src/acp_status.rs
line_hint: 203
author: 
created: 
title: "with_elapsed"
tags: []
signature: "pub fn with_elapsed (status : & StatusLine , elapsed : Duration) -> StatusLine"
---

Recounts running seconds on a status line for elapsed duration.

The line with its running seconds (`… · N s`) recounted for `elapsed`, so
the shell can tick a line it received without formatting anything.

# Examples

```rust
use crawrad_agent::acp_status::{StatusLine, with_elapsed};
use std::time::Duration;

let line = StatusLine {
    line: "Thinking…".to_string(),
    sub: "thought · 1 s".to_string(),
    done: false,
};
let ticked = with_elapsed(&line, Duration::from_secs(5));
assert!(ticked.sub.contains("5 s"));
```
