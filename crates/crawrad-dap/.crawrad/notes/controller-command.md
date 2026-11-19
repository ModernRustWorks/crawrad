---
id: controller-command
kind: doc
target_file: src/controller.rs
line_hint: 34
author: 
created: 
title: "Command"
tags: []
signature: "pub enum Command"
---

What the IDE asks of the session.

# Examples

```rust
use crawrad_dap::Command;

let cmd = Command::Continue;
assert_eq!(cmd, Command::Continue);
```
