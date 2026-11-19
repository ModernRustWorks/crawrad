---
id: acp-acpconfig-create-acp-client
kind: doc
target_file: src/acp.rs
line_hint: 515
author: 
created: 
title: "AcpConfig::create_acp_client"
tags: []
signature: "pub fn create_acp_client (& self , cwd : Option < & Path > ,) -> Result < super :: client :: AcpClient , String >"
---

Spawns a stateful, continuous ACP client connected to this configured provider.

# Errors

Returns an error if the agent is not installed or client spawning fails.

# Examples

```rust,no_run
// Spawns external ACP client process.
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
let client = config.create_acp_client(None).unwrap();
assert!(client.is_alive());
```
