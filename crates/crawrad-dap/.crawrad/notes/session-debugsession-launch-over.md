---
id: session-debugsession-launch-over
kind: doc
target_file: src/session.rs
line_hint: 160
author: 
created: 
title: "DebugSession::launch_over"
tags: []
signature: "pub fn launch_over < B : Clone + Into < SourceBreakpoint > > (connection : DapConnection , config : & LaunchConfig , breakpoints : & BTreeMap < PathBuf , Vec < B > > ,) -> Result < Self , DapError >"
---

The same over a connection that already exists (tests, remote adapters).

# Examples

```rust
use std::collections::BTreeMap;
use std::path::PathBuf;
use crawrad_dap::{DapConnection, DebugSession, LaunchConfig, SourceBreakpoint};

let (client_reader, _to_client) = std::io::pipe().unwrap();
let (_from_client, client_writer) = std::io::pipe().unwrap();
let conn = DapConnection::new(client_reader, client_writer);
let res = DebugSession::launch_over(
    conn,
    &LaunchConfig::default(),
    &BTreeMap::<PathBuf, Vec<SourceBreakpoint>>::new(),
);
assert!(res.is_err());
```

# Errors

Returns [`DapError`] if the DAP handshake or breakpoint initialization fails.
