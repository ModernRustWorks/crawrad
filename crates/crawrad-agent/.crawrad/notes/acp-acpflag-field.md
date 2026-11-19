---
id: acp-acpflag-field
kind: doc
target_file: src/acp.rs
line_hint: 179
author: 
created: 
title: "AcpFlag::field"
tags: []
signature: "pub fn field (self , c : & mut AcpConfig) -> & mut bool"
---

Its field in the configuration.

# Examples

```rust
use crawrad_agent::AcpConfig;
use crawrad_agent::acp::AcpFlag;

let mut config = AcpConfig::default();
*AcpFlag::AutoApproveReads.field(&mut config) = true;
assert!(config.auto_approve_reads);
```
