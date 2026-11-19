---
id: acp-acpconfig-create-acp-client-with-permissions
kind: doc
target_file: src/acp.rs
line_hint: 523
author: 
created: 
title: "AcpConfig::create_acp_client_with_permissions"
tags: []
signature: "pub fn create_acp_client_with_permissions (& self , cwd : Option < & Path > , perms : Option < std :: sync :: Arc < std :: sync :: Mutex < super :: permission :: SessionPermissions > > > ,) -> Result < super :: client :: AcpClient , String >"
---

Spawns an ACP client with optional permissions.

Spawns a stateful, continuous ACP client optionally sharing existing session permissions.

# Errors

Returns an error if the agent is not installed or client spawning fails.

# Examples

```rust,no_run
// Spawns external ACP client process.
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
let client = config.create_acp_client_with_permissions(None, None).unwrap();
assert!(client.is_alive());
```
