---
id: acp-acpconfig-test-connection-for
kind: doc
target_file: src/acp.rs
line_hint: 438
author: 
created: 
title: "AcpConfig::test_connection_for"
tags: []
signature: "pub fn test_connection_for (provider : & str , endpoint : & str) -> Result < String , String >"
---

Probes connection for a specific provider and endpoint.

Probes connection for a specific provider and endpoint without consuming LLM quota.

# Errors

Returns an error if the provider executable is not found or fails the probe.

# Examples

```rust,no_run
// Probes external agent executable.
use crawrad_agent::AcpConfig;

let version = AcpConfig::test_connection_for("Claude Agent", "stdio:claude-agent").unwrap();
assert!(!version.is_empty());
```
