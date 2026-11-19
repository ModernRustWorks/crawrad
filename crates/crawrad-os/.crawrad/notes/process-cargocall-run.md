---
id: process-cargocall-run
kind: doc
target_file: src/process.rs
line_hint: 92
author: 
created: 
title: "CargoCall::run"
tags: []
signature: "pub fn run (package : & str , dir : impl Into < PathBuf >) -> Self"
---

Run one package.

Run one package. Only ever [`detach`][crate::process::detach]ed — see the module header.

After a green `cargo build` this recompiles nothing: it just executes.

# Examples

```rust
use crawrad_os::process::CargoCall;

let call = CargoCall::run("crawrad-os", ".");
let cmd = call.command();
assert_eq!(cmd.get_program(), "cargo");
```
