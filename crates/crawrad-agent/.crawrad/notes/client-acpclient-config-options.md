---
id: client-acpclient-config-options
kind: doc
target_file: src/client.rs
line_hint: 1202
author: 
created: 
title: "AcpClient::config_options"
tags: []
signature: "pub fn config_options (& self) -> Vec < serde_json :: Value >"
---

Returns the active session's configuration options, or an empty list.

# Examples

```rust,no_run
// Queries config options of external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(client.config_options().is_empty());
```
