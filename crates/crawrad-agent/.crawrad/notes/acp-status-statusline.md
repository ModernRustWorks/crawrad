---
id: acp-status-statusline
kind: doc
target_file: src/acp_status.rs
line_hint: 19
author: 
created: 
title: "StatusLine"
tags: []
signature: "pub struct StatusLine"
---

One status line for the agent card.

One status line: the italic `line`, the monospace `sub`, and whether the
turn is over (drawn dimmed, with a grey bar).

# Examples

```rust
use crawrad_agent::acp_status::StatusLine;

let line = StatusLine::default();
assert!(!line.done);
```
