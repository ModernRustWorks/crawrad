---
id: acp-registry-registry-status
kind: doc
target_file: src/acp_registry.rs
line_hint: 611
author: 
created: 
title: "registry_status"
tags: []
signature: "pub fn registry_status () -> RegistryStatus"
---

Returns the current registry status.

# Panics

Panics if the internal registry store lock is poisoned.

# Examples

```rust
use crawrad_agent::acp_registry::{RegistryStatus, registry_status};

let status = registry_status();
assert!(matches!(status, RegistryStatus::Cached | RegistryStatus::Live | RegistryStatus::Empty | RegistryStatus::Offline));
```
