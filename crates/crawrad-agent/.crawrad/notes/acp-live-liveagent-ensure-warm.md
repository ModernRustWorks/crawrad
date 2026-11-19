---
id: acp-live-liveagent-ensure-warm
kind: doc
target_file: src/acp_live.rs
line_hint: 234
author: 
created: 
title: "LiveAgent::ensure_warm"
tags: []
signature: "pub fn ensure_warm (& mut self , config : & AcpConfig , cwd : Option < PathBuf > , perms : Arc < Mutex < SessionPermissions > > ,) -> bool"
---

Ensures the agent is warm for the given configuration.

Starts warming the agent `config` names for `cwd`, unless that very agent is
already live, warming, or Failed (a caller retries with `reset` first: this
runs on every layout pass). Another agent or folder replaces the old one.
Returns whether a warm-up started.

# Examples

```rust,no_run
// Spawns external ACP client and thread.
use crawrad_agent::{AcpConfig, LiveAgent, SessionPermissions};
use std::sync::{Arc, Mutex};

let config = AcpConfig::default();
let perms = Arc::new(Mutex::new(SessionPermissions::new()));
let mut agent = LiveAgent::new();
let started = agent.ensure_warm(&config, None, perms);
assert!(started);
```
