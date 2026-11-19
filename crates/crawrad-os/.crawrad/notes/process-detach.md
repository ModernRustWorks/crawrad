---
id: process-detach
kind: doc
target_file: src/process.rs
line_hint: 380
author: 
created: 
title: "detach"
tags: []
signature: "pub fn detach (cmd : & mut Command) -> Result < () , String >"
---

Start it and let go — the IDE stays alive while the child runs.

This is what separates *running* from *hung*: the user's form is open and
the IDE keeps answering, which is how a classic RAD IDE behaves. The child's output is
not captured, because there is nobody left to read it.

# Examples

```rust
use crawrad_os::process::{detach, no_console};

let mut cmd = no_console("cargo");
cmd.arg("--version");
assert!(detach(&mut cmd).is_ok());
```

# Errors

Returns an error if the process could not be launched.
