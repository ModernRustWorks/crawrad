---
id: agent-profile-query-agent-models
kind: doc
target_file: src/agent_profile.rs
line_hint: 428
author: 
created: 
title: "query_agent_models"
tags: []
signature: "pub fn query_agent_models (binary : & Path , subcommand : & str) -> Result < Vec < ModelSpec > , String >"
---

Queries the agent CLI for live models list (e.g. `agy models`).

# Errors

Returns an error if the agent CLI fails to execute or no models are parsed.

# Examples

```rust,no_run
// Executes external CLI binary.
use crawrad_agent::query_agent_models;
use std::path::Path;

let models = query_agent_models(Path::new("agy"), "models").unwrap();
assert!(!models.is_empty());
```
