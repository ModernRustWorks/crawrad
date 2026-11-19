---
id: client-acpclient-can-load-session
kind: doc
target_file: src/client.rs
line_hint: 1039
author: 
created: 
title: "AcpClient::can_load_session"
tags: []
signature: "pub fn can_load_session (& self) -> bool"
---

Whether the agent can resume a past session (`session/load`, #478).

# Examples

```rust,no_run
// Checks session resumption capability of external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(!client.can_load_session());
```
