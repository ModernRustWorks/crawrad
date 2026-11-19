---
id: acp-acpconfig-execute-prompt-with-session
kind: doc
target_file: src/acp.rs
line_hint: 594
author: 
created: 
title: "AcpConfig::execute_prompt_with_session"
tags: []
signature: "pub fn execute_prompt_with_session (& self , prompt : & str , session : Option < (& str , bool) > ,) -> Result < String , String >"
---

Executes a prompt query optionally bound to a multi-turn CLI session.

# Errors

Returns an error if prompt execution fails or the process exits abnormally.

# Examples

```rust,no_run
// Executes external CLI agent prompt.
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
let reply = config.execute_prompt_with_session("hello", None).unwrap();
assert!(!reply.is_empty());
```
