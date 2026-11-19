---
id: client-acpclient-spawn-full
kind: doc
target_file: src/client.rs
line_hint: 298
author: 
created: 
title: "AcpClient::spawn_full"
tags: []
signature: "pub fn spawn_full (binary_path : & Path , args : & [String] , cwd : Option < & Path > , auto_approve_reads : bool , allow_native_agent_tools : bool , external_session_permissions : Option < Arc < Mutex < super :: permission :: SessionPermissions > > > ,) -> Result < Self , String >"
---

Spawns an ACP client with explicit permissions and policies.

Spawns an ACP client with explicit native tools policy and optional shared session permissions.

# Errors

Returns an error if spawning the process or setting up stdio fails.

# Examples

```rust,no_run
// Spawns external agent process.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn_full(Path::new("agy"), &[], None, false, false, None).unwrap();
assert!(client.is_alive());
```
