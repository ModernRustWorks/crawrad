---
id: acp-registry-current-platform
kind: doc
target_file: src/acp_registry.rs
line_hint: 58
author: 
created: 
title: "current_platform"
tags: []
signature: "pub fn current_platform () -> & 'static str"
---

Returns the registry distribution platform key for the current build.

# Examples

```rust
use crawrad_agent::current_platform;

assert!(!current_platform().is_empty());
```
