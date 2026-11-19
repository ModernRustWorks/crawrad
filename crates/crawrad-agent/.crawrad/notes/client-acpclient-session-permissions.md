---
id: client-acpclient-session-permissions
kind: doc
target_file: src/client.rs
line_hint: 1430
author: 
created: 
title: "AcpClient::session_permissions"
tags: []
signature: "pub fn session_permissions (& self) -> Arc < Mutex < super :: permission :: SessionPermissions > >"
---

Returns the session permissions rules container.

# Examples

```rust,no_run
// Retrieves session permissions from client.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
let perms = client.session_permissions();
```
