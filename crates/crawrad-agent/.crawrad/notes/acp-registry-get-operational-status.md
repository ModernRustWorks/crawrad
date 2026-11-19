---
id: acp-registry-get-operational-status
kind: doc
target_file: src/acp_registry.rs
line_hint: 781
author: 
created: 
title: "get_operational_status"
tags: []
signature: "pub fn get_operational_status (cfg : & crate :: AcpConfig , custom_status_msg : Option < & str > ,) -> AcpAgentOperationalStatus"
---

Resolves operational status for the active ACP configuration.

# Examples

```rust
use crawrad_agent::{AcpConfig, acp_registry::get_operational_status};

let cfg = AcpConfig::default();
let status = get_operational_status(&cfg, None);
assert_eq!(status.active_name, "Claude Agent");
```
