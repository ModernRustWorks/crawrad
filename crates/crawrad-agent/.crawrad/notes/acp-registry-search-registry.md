---
id: acp-registry-search-registry
kind: doc
target_file: src/acp_registry.rs
line_hint: 588
author: 
created: 
title: "search_registry"
tags: []
signature: "pub fn search_registry (query : & str) -> Vec < AcpAgentEntry >"
---

Returns all agents in the registry matching the query.

Returns all agents in the registry matching the search query.
Case-insensitive match on name, id, author, description, and tags.

# Panics

Panics if the internal registry store lock is poisoned.

# Examples

```rust,no_run
// Spawns background thread to refresh registry over network.
use crawrad_agent::search_registry;

let agents = search_registry("claude");
assert_eq!(agents.len(), search_registry("claude").len());
```
