---
id: process-cargocall-build
kind: doc
target_file: src/process.rs
line_hint: 77
author: 
created: 
title: "CargoCall::build"
tags: []
signature: "pub fn build (package : & str , dir : impl Into < PathBuf >) -> Self"
---

Compile one package, asking for machine-readable diagnostics.

`--message-format=json` puts one JSON line per diagnostic on **standard
output**, with file, line and column. It is what turns "one error line
in the status bar" into a list that takes you to the place. The
`-diagnostic-rendered-ansi` variant was measured and dropped: it returns
the same text with colour escapes inside, and we have no ANSI
interpreter to undo them.

At the **root** of the workspace with `-p <member>`: one `Cargo.lock`
and one `target/` for the whole group, which is how Cargo treats a
workspace. Compiling from inside the member would give the same result
plus a second `target/` in the way of anyone with two members.

# Examples

```rust
use crawrad_os::process::CargoCall;

let call = CargoCall::build("crawrad-os", ".");
let cmd = call.command();
assert_eq!(cmd.get_program(), "cargo");
```
