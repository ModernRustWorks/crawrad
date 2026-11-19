---
id: debugcommand
kind: doc
target_file: src/lib.rs
line_hint: 732
author: 
created: 
title: "DebugCommand"
tags: []
signature: "pub enum DebugCommand"
---

Debugging commands requested by actions to the host.

The action only ASKS: whoever knows which file is in front and which line the
cursor is on is the shell, and the state of breakpoints lives in `crawrad-host`.

# Examples

```rust
use crawrad_actions::DebugCommand;

let cmd = DebugCommand::StepOver;
assert_eq!(cmd, DebugCommand::StepOver);
```
