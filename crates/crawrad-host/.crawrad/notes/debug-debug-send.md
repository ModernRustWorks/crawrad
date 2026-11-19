---
id: debug-debug-send
kind: doc
target_file: src/debug.rs
line_hint: 341
author: 
created: 
title: "Debug::send"
tags: []
signature: "pub fn send (& mut self , command : Command) -> bool"
---

Sends a command to the session; `false` when there is none.

A command that resumes a stopped program takes the stop away AT ONCE:
an agent that steps and reads `debug.state` right after must not read
the old place as if it were the new one (measured through the MCP on
30/09/2026 — every read lagged one step behind).

# Examples

```rust
use crawrad_dap::Command;
use crawrad_host::Debug;

let mut debug = Debug::default();
assert!(!debug.send(Command::Continue));
```
