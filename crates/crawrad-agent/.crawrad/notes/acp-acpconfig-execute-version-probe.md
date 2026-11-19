---
id: acp-acpconfig-execute-version-probe
kind: doc
target_file: src/acp.rs
line_hint: 477
author: 
created: 
title: "AcpConfig::execute_version_probe"
tags: []
signature: "pub fn execute_version_probe (binary : & str) -> Result < String , String >"
---

Probes CLI version for an agent binary without consuming LLM quota or token.

# Errors

Returns an error if the binary cannot be found or fails during execution.

# Examples

```rust,no_run
// Probes external agent binary.
use crawrad_agent::AcpConfig;

let version = AcpConfig::execute_version_probe("claude").unwrap();
assert!(!version.is_empty());
```
