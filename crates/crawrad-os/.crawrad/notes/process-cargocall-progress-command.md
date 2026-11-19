---
id: process-cargocall-progress-command
kind: doc
target_file: src/process.rs
line_hint: 130
author: 
created: 
title: "CargoCall::progress_command"
tags: []
signature: "pub fn progress_command (& self) -> Command"
---

The same call, asking cargo to draw its progress bar even into a pipe.

The same call, asking cargo to draw its `Building [..] N/M: crates` bar even
into a pipe, so a streamed run can show N / M. Only for [`stream`][crate::process::stream].

# Examples

```rust
use crawrad_os::process::CargoCall;

let call = CargoCall::build("crawrad-os", ".");
let cmd = call.progress_command();
assert_eq!(cmd.get_program(), "cargo");
```
