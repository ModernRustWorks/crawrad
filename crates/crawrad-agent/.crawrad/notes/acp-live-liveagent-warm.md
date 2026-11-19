---
id: acp-live-liveagent-warm
kind: doc
target_file: src/acp_live.rs
line_hint: 86
author: 
created: 
title: "LiveAgent::warm"
tags: []
signature: "pub fn warm (config : & AcpConfig , cwd : Option < PathBuf > , mcp_servers : Vec < Value > , perms : Arc < Mutex < SessionPermissions > > ,) -> WarmRx"
---

Spawns a thread connecting and initializing an ACP session.

Spawns a background thread that connects, initializes, and establishes a session.

# Examples

```rust,no_run
// Spawns external ACP client and thread.
use crawrad_agent::{AcpConfig, LiveAgent, SessionPermissions};
use std::sync::{Arc, Mutex};

let config = AcpConfig::default();
let perms = Arc::new(Mutex::new(SessionPermissions::new()));
let rx = LiveAgent::warm(&config, None, Vec::new(), perms);
```
