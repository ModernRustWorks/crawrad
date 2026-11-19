---
id: acp-acpconfig-build-invocation-for-prompt
kind: doc
target_file: src/acp.rs
line_hint: 603
author: 
created: 
title: "AcpConfig::build_invocation_for_prompt"
tags: []
signature: "pub fn build_invocation_for_prompt (& self , prompt : & str , session : Option < (& str , bool) > , cwd : Option < & Path > ,) -> Result < CliInvocation , String >"
---

Builds the CLI invocation for a prompt.

Builds the CLI invocation for a prompt according to the configured agent profile and endpoint.

# Errors

Returns an error if the endpoint is empty or direct execution is unsupported.

# Examples

```rust
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
let inv = config.build_invocation_for_prompt("hello", None, None).unwrap();
assert_eq!(inv.binary, "claude");
```
