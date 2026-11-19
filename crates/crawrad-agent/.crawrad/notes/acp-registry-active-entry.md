---
id: acp-registry-active-entry
kind: doc
target_file: src/acp_registry.rs
line_hint: 686
author: 
created: 
title: "active_entry"
tags: []
signature: "pub fn active_entry < 'a > (cfg : & crate :: AcpConfig , agents : & 'a [AcpAgentEntry] ,) -> Option < & 'a AcpAgentEntry >"
---

Resolves the registry entry of the agent configured in cfg.

The registry entry of the agent `cfg` launches — the same resolution the
launch uses (#561): the agent's profile by id names its registry id
(`codex` → `codex-acp`). Failing that, the entry whose command or
provider is the configured one.

# Examples

```rust
use crawrad_agent::{AcpConfig, acp_registry::active_entry};

let cfg = AcpConfig::default();
let entry = active_entry(&cfg, &[]);
assert!(entry.is_none());
```
