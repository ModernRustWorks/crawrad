---
id: acp-acpconfig-execute-prompt-with-session-and-cwd
kind: doc
target_file: src/acp.rs
line_hint: 649
author: 
created: 
title: "AcpConfig::execute_prompt_with_session_and_cwd"
tags: []
signature: "pub fn execute_prompt_with_session_and_cwd (& self , prompt : & str , session : Option < (& str , bool) > , cwd : Option < & Path > ,) -> Result < String , String >"
---

Executes a prompt query with session and working directory.

Executes a prompt query optionally bound to a multi-turn CLI session and working directory.

# Errors

Returns an error if invocation building fails or the process exits abnormally.

# Examples

```rust,no_run
// Executes external CLI agent prompt.
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
let reply = config.execute_prompt_with_session_and_cwd("hello", None, None).unwrap();
assert!(!reply.is_empty());
```
