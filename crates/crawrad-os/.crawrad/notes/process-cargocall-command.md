---
id: process-cargocall-command
kind: doc
target_file: src/process.rs
line_hint: 102
author: 
created: 
title: "CargoCall::command"
tags: []
signature: "pub fn command (& self) -> Command"
---

The command, built the way this house talks to cargo.

The three environment lines below are invisible when they are right, and
each one of them has already cost a defect. A test that reads them is
what prevents the fourth.

# Examples

```rust
use crawrad_os::process::CargoCall;

let call = CargoCall::build("crawrad-os", ".");
let cmd = call.command();
assert_eq!(cmd.get_program(), "cargo");
```
