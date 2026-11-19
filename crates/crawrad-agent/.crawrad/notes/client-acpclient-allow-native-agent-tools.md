---
id: client-acpclient-allow-native-agent-tools
kind: doc
target_file: src/client.rs
line_hint: 1467
author: 
created: 
title: "AcpClient::allow_native_agent_tools"
tags: []
signature: "pub fn allow_native_agent_tools (& self) -> bool"
---

Whether native agent CLI tools are allowed without restriction.

# Examples

```rust,no_run
// Checks native tool allowance on client.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(!client.allow_native_agent_tools());
```
