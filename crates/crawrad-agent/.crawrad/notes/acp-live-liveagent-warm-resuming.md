---
id: acp-live-liveagent-warm-resuming
kind: doc
target_file: src/acp_live.rs
line_hint: 96
author: 
created: 
title: "LiveAgent::warm_resuming"
tags: []
signature: "pub fn warm_resuming (config : & AcpConfig , cwd : Option < PathBuf > , mcp_servers : Vec < Value > , perms : Arc < Mutex < SessionPermissions > > , resume : Option < String > ,) -> WarmRx"
---

Warms an ACP agent, resuming an existing session if possible.

As [`Self::warm`], resuming the ACP session `resume` (`session/load`) when
the agent declared `loadSession`; otherwise, or if loading fails, a new one.

# Examples

```rust,no_run
// Spawns external ACP client and thread.
use crawrad_agent::{AcpConfig, LiveAgent, SessionPermissions};
use std::sync::{Arc, Mutex};

let config = AcpConfig::default();
let perms = Arc::new(Mutex::new(SessionPermissions::new()));
let rx = LiveAgent::warm_resuming(&config, None, Vec::new(), perms, None);
```
