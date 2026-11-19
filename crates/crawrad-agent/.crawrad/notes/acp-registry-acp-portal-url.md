---
id: acp-registry-acp-portal-url
kind: doc
target_file: src/acp_registry.rs
line_hint: 19
author: 
created: 
title: "ACP_PORTAL_URL"
tags: []
signature: "pub const ACP_PORTAL_URL : & str = \"https://agentclientprotocol.com\""
---

Base URL of the ACP portal (browsable by humans).

# Examples

```rust
use crawrad_agent::acp_registry::ACP_PORTAL_URL;

assert!(ACP_PORTAL_URL.starts_with("https://"));
```
