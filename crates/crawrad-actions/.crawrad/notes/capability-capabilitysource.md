---
id: capability-capabilitysource
kind: doc
target_file: src/capability.rs
line_hint: 19
author: 
created: 
title: "CapabilitySource"
tags: []
signature: "pub enum CapabilitySource"
---

The origin of a capability in the registry.

# Examples

```rust
use crawrad_actions::capability::CapabilitySource;

let source = CapabilitySource::Native;
assert_eq!(source.to_string(), "native");
```
