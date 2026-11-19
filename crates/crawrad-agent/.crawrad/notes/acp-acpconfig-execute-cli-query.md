---
id: acp-acpconfig-execute-cli-query
kind: doc
target_file: src/acp.rs
line_hint: 660
author: 
created: 
title: "AcpConfig::execute_cli_query"
tags: []
signature: "pub fn execute_cli_query (binary : & str , prompt : & str) -> Result < String , String >"
---

Runs a query non-interactively via the CLI tool with null stdin.

# Errors

Returns an error if the process fails to spawn or returns non-zero status.

# Examples

```rust,no_run
// Executes external CLI agent query.
use crawrad_agent::AcpConfig;

let reply = AcpConfig::execute_cli_query("claude", "hello").unwrap();
assert!(!reply.is_empty());
```
