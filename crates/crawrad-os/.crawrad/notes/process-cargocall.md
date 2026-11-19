---
id: process-cargocall
kind: doc
target_file: src/process.rs
line_hint: 58
author: 
created: 
title: "CargoCall"
tags: []
signature: "pub struct CargoCall"
---

A call to `cargo`, described once.

It exists so that a path and an argument list are never spelled out twice.
Issue #67 was born exactly that way: three `Command::new("cargo")` grew
beside the shared builder, each with its own idea of which environment
variables to clear, and they drifted. A value that can be compared in a test
is what keeps `build -p x --message-format=json` honest — the constructors
below are the only place that string is written.

# Examples

```rust
use crawrad_os::process::CargoCall;

let call = CargoCall::build("my-package", ".");
let cmd = call.command();
assert_eq!(cmd.get_program(), "cargo");
```
