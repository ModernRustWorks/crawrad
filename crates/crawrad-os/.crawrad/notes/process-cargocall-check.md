---
id: process-cargocall-check
kind: doc
target_file: src/process.rs
line_hint: 82
author: 
created: 
title: "CargoCall::check"
tags: []
signature: "pub fn check (package : & str , dir : impl Into < PathBuf >) -> Self"
---

Check one package for errors/warnings without full code generation.

# Examples

```rust
use crawrad_os::process::CargoCall;

let call = CargoCall::check("crawrad-os", ".");
let cmd = call.command();
assert_eq!(cmd.get_program(), "cargo");
```
