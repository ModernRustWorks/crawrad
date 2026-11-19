---
id: acp-acpconfig-execute-prompt
kind: doc
target_file: src/acp.rs
line_hint: 510
author: 
created: 
title: "AcpConfig::execute_prompt"
tags: []
signature: "pub fn execute_prompt (& self , prompt : & str) -> Result < String , String >"
---

Executes a prompt query using the active agent CLI according to its profile.

# Errors

Returns an error if prompt invocation fails or process execution fails.

# Examples

```rust,no_run
// Executes external CLI agent prompt.
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
let reply = config.execute_prompt("hello").unwrap();
assert!(!reply.is_empty());
```
