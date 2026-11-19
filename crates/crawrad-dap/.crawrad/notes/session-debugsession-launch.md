---
id: session-debugsession-launch
kind: doc
target_file: src/session.rs
line_hint: 150
author: 
created: 
title: "DebugSession::launch"
tags: []
signature: "pub fn launch < B : Clone + Into < SourceBreakpoint > > (adapter : & Path , config : & LaunchConfig , breakpoints : & BTreeMap < PathBuf , Vec < B > > ,) -> Result < Self , DapError >"
---

Starts an adapter and launches the program under debug.

Starts `adapter` and launches `config` under it, with `breakpoints`
set before the program runs.

# Examples

```rust
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use crawrad_dap::{DebugSession, LaunchConfig, SourceBreakpoint};

let res = DebugSession::launch(
    Path::new("nonexistent-adapter"),
    &LaunchConfig::default(),
    &BTreeMap::<PathBuf, Vec<SourceBreakpoint>>::new(),
);
assert!(res.is_err());
```

# Errors

Returns [`DapError`] if spawning the adapter or the DAP handshake fails.
