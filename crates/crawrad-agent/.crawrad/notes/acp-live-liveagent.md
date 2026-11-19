---
id: acp-live-liveagent
kind: doc
target_file: src/acp_live.rs
line_hint: 33
author: 
created: 
title: "LiveAgent"
tags: []
signature: "pub struct LiveAgent"
---

The stateful live agent container held by the shell's chat state.

# Examples

```rust
use crawrad_agent::LiveAgent;

let agent = LiveAgent::new();
assert_eq!(agent.agent_key(), "");
```
