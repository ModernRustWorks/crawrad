---
id: acp-registry-acplaunch
kind: doc
target_file: src/acp_registry.rs
line_hint: 51
author: 
created: 
title: "AcpLaunch"
tags: []
signature: "pub struct AcpLaunch"
---

Structured ACP launch parameters.

# Examples

```rust
use crawrad_agent::AcpLaunch;

let launch = AcpLaunch {
    program: "npx".to_string(),
    args: vec!["-y".to_string(), "my-agent".to_string()],
};
assert_eq!(launch.program, "npx");
```
