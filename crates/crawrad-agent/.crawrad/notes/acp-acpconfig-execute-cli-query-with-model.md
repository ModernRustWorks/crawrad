---
id: acp-acpconfig-execute-cli-query-with-model
kind: doc
target_file: src/acp.rs
line_hint: 665
author: 
created: 
title: "AcpConfig::execute_cli_query_with_model"
tags: []
signature: "pub fn execute_cli_query_with_model (binary : & str , prompt : & str , model : Option < & str > ,) -> Result < String , String >"
---

Runs a query non-interactively via the CLI tool with an optional model.

Runs a query non-interactively via the CLI tool with an optional model parameter.

# Errors

Returns an error if the process fails to spawn or returns non-zero status.

# Examples

```rust,no_run
// Executes external CLI agent query.
use crawrad_agent::AcpConfig;

let reply = AcpConfig::execute_cli_query_with_model("claude", "hello", Some("sonnet")).unwrap();
assert!(!reply.is_empty());
```
