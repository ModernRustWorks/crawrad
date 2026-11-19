---
id: build-settings-buildstate-cargo-call
kind: doc
target_file: src/build_settings.rs
line_hint: 488
author: 
created: 
title: "BuildState::cargo_call"
tags: []
signature: "pub fn cargo_call (& self , cmd : CargoCommand , package : & str , root : & Path ,) -> (crawrad_os :: process :: CargoCall , String)"
---

Builds the cargo call and command-line string for the given command.

Builds the CargoCall and the displayed command-line string for the given command.

# Examples

```rust
use std::path::Path;
use crawrad_project::build_settings::{BuildState, CargoCommand};

let state = BuildState::default();
let (_call, cmd_line) = state.cargo_call(CargoCommand::Build, "my-app", Path::new("."));
assert!(cmd_line.contains("cargo build -p my-app"));
```
