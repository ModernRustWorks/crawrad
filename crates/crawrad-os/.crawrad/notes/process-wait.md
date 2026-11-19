---
id: process-wait
kind: doc
target_file: src/process.rs
line_hint: 207
author: 
created: 
title: "wait"
tags: []
signature: "pub fn wait (cmd : & mut Command) -> Result < Outcome , String >"
---

Run to completion and keep everything.

`&mut Command` and not `Command` so a builder chain reads straight through:
`wait(no_console("git").arg("status"))`.

The `Err` is for the process that never started — the program is not on the
`PATH`, the directory does not exist. A child that ran and failed is `Ok`
with `success: false`, because *it answered*, and its answer is in
[`Outcome::stderr`]. Collapsing the two would lose that distinction, and the
two need different sentences on screen.

# Examples

```rust
use crawrad_os::process::{no_console, wait};

let mut cmd = no_console("cargo");
cmd.arg("--version");
let outcome = wait(&mut cmd).unwrap();
assert!(outcome.success);
```

# Errors

Returns an error if the process could not be spawned (for example, program not found).
