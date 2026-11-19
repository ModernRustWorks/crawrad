---
id: slash-commands-acp-probe
kind: doc
target_file: src/slash_commands.rs
line_hint: 287
author: 
created: 
title: "acp_probe"
tags: []
signature: "pub fn acp_probe (target : & str , configured : & crate :: acp :: AcpConfig) -> Result < String , String >"
---

Probes the agent connection for `/acp test`.

`/acp test [target]`: probes the agent connection.

# Examples

```rust,no_run
// Probing runs an external agent binary.
use crawrad_agent::acp::AcpConfig;
use crawrad_agent::slash_commands::acp_probe;

let cfg = AcpConfig::default();
let res = acp_probe("claude", &cfg).unwrap();
assert!(!res.is_empty());
```

# Errors

Returns an error if the agent executable is not found or fails the probe.
