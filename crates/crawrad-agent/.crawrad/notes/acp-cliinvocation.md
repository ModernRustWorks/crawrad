---
id: acp-cliinvocation
kind: doc
target_file: src/acp.rs
line_hint: 19
author: 
created: 
title: "CliInvocation"
tags: []
signature: "pub struct CliInvocation"
---

A structured CLI invocation definition with arguments and stdin payload.

# Examples

```rust
use crawrad_agent::CliInvocation;

let inv = CliInvocation {
    binary: "claude".to_string(),
    args: vec!["-p".to_string(), "hello".to_string()],
    stdin_data: String::new(),
    cwd: None,
};
assert_eq!(inv.binary, "claude");
```
