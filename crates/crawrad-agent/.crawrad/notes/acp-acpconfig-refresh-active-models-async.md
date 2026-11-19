---
id: acp-acpconfig-refresh-active-models-async
kind: doc
target_file: src/acp.rs
line_hint: 408
author: 
created: 
title: "AcpConfig::refresh_active_models_async"
tags: []
signature: "pub fn refresh_active_models_async (& self)"
---

Refreshes the model catalog for the active provider in a background thread.

# Examples

```rust,no_run
// Spawns a background thread to query external agent CLI.
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
config.refresh_active_models_async();
assert_eq!(config.agent_id, "claude");
```
