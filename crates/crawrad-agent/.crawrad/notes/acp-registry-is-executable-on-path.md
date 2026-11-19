---
id: acp-registry-is-executable-on-path
kind: doc
target_file: src/acp_registry.rs
line_hint: 188
author: 
created: 
title: "is_executable_on_path"
tags: []
signature: "pub fn is_executable_on_path (binary_name : & str) -> bool"
---

Helper that checks whether an executable exists on the system PATH.

# Examples

```rust
use crawrad_agent::acp_registry::is_executable_on_path;

assert!(!is_executable_on_path("non_existent_binary_12345"));
```
