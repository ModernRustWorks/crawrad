---
id: process-cargocall-clean
kind: doc
target_file: src/process.rs
line_hint: 87
author: 
created: 
title: "CargoCall::clean"
tags: []
signature: "pub fn clean (package : & str , dir : impl Into < PathBuf >) -> Self"
---

Clean build artifacts for one package.

# Examples

```rust
use crawrad_os::process::CargoCall;

let call = CargoCall::clean("crawrad-os", ".");
let cmd = call.command();
assert_eq!(cmd.get_program(), "cargo");
```
