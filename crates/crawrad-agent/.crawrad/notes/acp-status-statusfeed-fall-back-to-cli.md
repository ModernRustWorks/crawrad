---
id: acp-status-statusfeed-fall-back-to-cli
kind: doc
target_file: src/acp_status.rs
line_hint: 244
author: 
created: 
title: "StatusFeed::fall_back_to_cli"
tags: []
signature: "pub fn fall_back_to_cli (& mut self)"
---

Restarts the status line using CLI wording.

ACP could not be reached: the line restarts on the CLI wording, keeping
the clock of the prompt.

# Examples

```rust
use crawrad_agent::acp_status::StatusFeed;
use std::sync::mpsc::channel;

let (tx, _rx) = channel();
let mut feed = StatusFeed::start("claude", false, tx);
feed.fall_back_to_cli();
```
