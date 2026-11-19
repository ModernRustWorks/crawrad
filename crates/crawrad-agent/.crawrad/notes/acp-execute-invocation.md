---
id: acp-execute-invocation
kind: doc
target_file: src/acp.rs
line_hint: 80
author: 
created: 
title: "execute_invocation"
tags: []
signature: "pub fn execute_invocation (invocation : & CliInvocation , resolve_bin : impl Fn (& str) -> PathBuf ,) -> Result < String , String >"
---

Executes a CLI invocation and captures its output.

Executes a CLI invocation, streaming prompt via stdin and capturing stdout/stderr cleanly.

# Errors

Returns an error if the process fails to spawn, stdin write fails, waiting fails, or exit status is non-zero.

# Examples

```rust,no_run
// Does not execute external binaries during automated documentation tests.
use crawrad_agent::{CliInvocation, execute_invocation};
use std::path::PathBuf;

let inv = CliInvocation {
    binary: "echo".to_string(),
    args: vec!["ok".to_string()],
    stdin_data: String::new(),
    cwd: None,
};
let res = execute_invocation(&inv, |bin| PathBuf::from(bin));
assert!(res.is_ok());
```
