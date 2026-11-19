---
id: acp-registry-registry-subtitle
kind: doc
target_file: src/acp_registry.rs
line_hint: 621
author: 
created: 
title: "registry_subtitle"
tags: []
signature: "pub fn registry_subtitle () -> String"
---

Returns the subtitle string for the registry header.

Returns the subtitle string for the registry header.
Example: "41 agents · updated 2h ago" or "offline — showing cached list".

# Examples

```rust
use crawrad_agent::acp_registry::registry_subtitle;

let sub = registry_subtitle();
assert!(!sub.is_empty());
```
