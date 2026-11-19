---
id: acp-live-turnrequest
kind: doc
target_file: src/acp_live.rs
line_hint: 308
author: 
created: 
title: "TurnRequest"
tags: []
signature: "pub struct TurnRequest"
---

One chat turn's inputs, moved into the worker thread.

# Examples

```rust
use crawrad_agent::{AcpConfig, SessionPermissions};
use crawrad_agent::acp_live::TurnRequest;
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
assert_eq!(req.agent, "claude");
```
