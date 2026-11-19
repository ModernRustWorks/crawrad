---
id: acp-acpconfig-test-connection
kind: doc
target_file: src/acp.rs
line_hint: 433
author: 
created: 
title: "AcpConfig::test_connection"
tags: []
signature: "pub fn test_connection (& self) -> Result < String , String >"
---

Probes the connection to the configured ACP provider without consuming quota.

# Errors

Returns an error if the agent executable cannot be found or fails to run.

# Examples

```rust,no_run
// Probes external agent executable.
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
let version = config.test_connection().unwrap();
assert!(!version.is_empty());
```
