---
id: client-acpclient-clear-session-permissions
kind: doc
target_file: src/client.rs
line_hint: 1460
author: 
created: 
title: "AcpClient::clear_session_permissions"
tags: []
signature: "pub fn clear_session_permissions (& self)"
---

Clears all session-wide permissions rules.

# Examples

```rust,no_run
// Clears session permissions on client.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
client.clear_session_permissions();
```
