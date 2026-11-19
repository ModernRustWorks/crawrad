---
id: acp-registry-registry-agent-count
kind: doc
target_file: src/acp_registry.rs
line_hint: 616
author: 
created: 
title: "registry_agent_count"
tags: []
signature: "pub fn registry_agent_count () -> usize"
---

Returns the total number of agents currently loaded.

# Panics

Panics if the internal registry store lock is poisoned.

# Examples

```rust
use crawrad_agent::acp_registry::{RegistryStatus, registry_agent_count, registry_status};

let count = registry_agent_count();
if registry_status() != RegistryStatus::Empty {
    assert!(count > 0);
} else {
    assert_eq!(count, 0);
}
```
