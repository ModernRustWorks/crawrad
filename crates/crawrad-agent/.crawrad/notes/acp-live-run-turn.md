---
id: acp-live-run-turn
kind: doc
target_file: src/acp_live.rs
line_hint: 328
author: 
created: 
title: "run_turn"
tags: []
signature: "pub fn run_turn (req : TurnRequest , is_cancelled : impl Fn () -> bool , on_perm_request : impl Fn (Arc < PermissionRequest >) , on_write : impl FnMut (Arc < FileWriteRequest >) ,) -> TurnResult"
---

Runs one turn on live client or falls back to CLI.

Runs one turn: on the live client when there is one, else on a client made for
this turn, and when ACP cannot be reached, through the CLI, which says so.

# Examples

```rust,no_run
// Executes external agent CLI turn.
use crawrad_agent::{AcpConfig, SessionPermissions};
use crawrad_agent::acp_live::{TurnRequest, run_turn};
use std::collections::BTreeMap;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

let (tx, _rx) = channel();
let req = TurnRequest {
    config: AcpConfig::default(),
    live: None,
    prompt: "hello".to_string(),
    images: Vec::new(),
    cwd: None,
    buffers: BTreeMap::new(),
    perms: Arc::new(Mutex::new(SessionPermissions::new())),
    cli_session: None,
    agent: "claude".to_string(),
    status: tx,
};
let res = run_turn(req, || false, |_| {}, |_| {});
assert!(res.is_ok());
```
