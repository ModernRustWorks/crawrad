---
id: acp-status-statusfeed-start
kind: doc
target_file: src/acp_status.rs
line_hint: 222
author: 
created: 
title: "StatusFeed::start"
tags: []
signature: "pub fn start (agent : & str , via_cli : bool , tx : Sender < StatusLine >) -> Self"
---

Starts the line (step 1) and sends it.

# Examples

```rust
use crawrad_agent::acp_status::StatusFeed;
use std::sync::mpsc::channel;

let (tx, _rx) = channel();
let feed = StatusFeed::start("claude", false, tx);
```
