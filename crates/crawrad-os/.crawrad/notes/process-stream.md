---
id: process-stream
kind: doc
target_file: src/process.rs
line_hint: 229
author: 
created: 
title: "stream"
tags: []
signature: "pub fn stream (cmd : & mut Command , mut on_line : impl FnMut (Pipe , & str) , cancelled : impl Fn () -> bool ,) -> Result < Outcome , String >"
---

Runs a command to completion, streaming each line to `on_line` as it arrives.

Run to completion, handing every line to `on_line` AS IT ARRIVES, and keep
everything like `wait` does. `cancelled` is polled (about every 50 ms); when
it returns true the child is killed and the result is `Ok` with
`success: false` and `cancelled: true`.

This exists so the user sees live compiler progress (which crate is currently
compiling, N / M counter, and console output) instead of freezing on a modal
spinner until the child exits.

# Examples

```rust
use crawrad_os::process::{no_console, stream};

let mut cmd = no_console("cargo");
cmd.arg("--version");
let mut lines = Vec::new();
let outcome = stream(&mut cmd, |_pipe, line| lines.push(line.to_string()), || false).unwrap();
assert!(outcome.success);
assert!(!lines.is_empty());
```

# Errors

Returns an error if spawning the process or taking its stdio pipes fails.
