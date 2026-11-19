---
id: acp-acpflag-get
kind: doc
target_file: src/acp.rs
line_hint: 171
author: 
created: 
title: "AcpFlag::get"
tags: []
signature: "pub fn get (self , c : & AcpConfig) -> bool"
---

Whether the box is checked.

# Examples

```rust
use crawrad_agent::AcpConfig;
use crawrad_agent::acp::AcpFlag;

let config = AcpConfig::default();
assert!(AcpFlag::AutoApproveReads.get(&config));
```
