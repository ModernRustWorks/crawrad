---
id: process-cargocall-new
kind: doc
target_file: src/process.rs
line_hint: 66
author: 
created: 
title: "CargoCall::new"
tags: []
signature: "pub fn new < S : Into < String > > (args : impl IntoIterator < Item = S > , dir : impl Into < PathBuf > ,) -> Self"
---

An arbitrary call — `cargo <args>` inside `dir`.

# Examples

```rust
use crawrad_os::process::CargoCall;

let call = CargoCall::new(["version"], ".");
let cmd = call.command();
assert_eq!(cmd.get_program(), "cargo");
```
