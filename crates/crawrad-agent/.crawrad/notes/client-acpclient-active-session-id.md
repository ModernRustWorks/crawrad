---
id: client-acpclient-active-session-id
kind: doc
target_file: src/client.rs
line_hint: 1197
author: 
created: 
title: "AcpClient::active_session_id"
tags: []
signature: "pub fn active_session_id (& self) -> Option < String >"
---

Returns the currently active session ID, if established.

# Examples

```rust,no_run
// Queries active session id of external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(client.active_session_id().is_none());
```
