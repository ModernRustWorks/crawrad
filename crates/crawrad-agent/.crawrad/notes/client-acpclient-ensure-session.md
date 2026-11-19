---
id: client-acpclient-ensure-session
kind: doc
target_file: src/client.rs
line_hint: 1183
author: 
created: 
title: "AcpClient::ensure_session"
tags: []
signature: "pub fn ensure_session (& self , cwd : Option < & Path > , mcp_servers : Vec < serde_json :: Value > ,) -> Result < String , String >"
---

Ensures an active session exists, creating one if necessary.

Ensures an active session exists; reuses existing session ID for multi-turn wire continuity.

# Errors

Returns an error if creating a new session fails.

# Examples

```rust,no_run
// Ensures session exists on external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
let id = client.ensure_session(None, vec![]).unwrap();
assert!(!id.is_empty());
```
