---
id: acp-registry-acpagentoperationalstatus
kind: doc
target_file: src/acp_registry.rs
line_hint: 674
author: 
created: 
title: "AcpAgentOperationalStatus"
tags: []
signature: "pub struct AcpAgentOperationalStatus"
---

Operational and presentation status of the active ACP CLI agent.

# Examples

```rust
use crawrad_agent::acp_registry::AcpAgentOperationalStatus;

let status = AcpAgentOperationalStatus {
    active_name: "Claude".to_string(),
    active_version: "1.0".to_string(),
    command: "claude".to_string(),
    status_title: "Ready".to_string(),
    status_detail: "Detail".to_string(),
    target_url: "https://example.com".to_string(),
    is_installed: true,
};
assert!(status.is_installed);
```
