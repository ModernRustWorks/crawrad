---
id: agent-profile-resolve-binary-path
kind: doc
target_file: src/agent_profile.rs
line_hint: 648
author: 
created: 
title: "resolve_binary_path"
tags: []
signature: "pub fn resolve_binary_path (binary : & str) -> PathBuf"
---

Resolves the absolute path of CLI binaries.

Resolves the absolute path of CLI binaries using PATH first, then OS-specific standard locations.

# Examples

```rust
use crawrad_agent::resolve_binary_path;

let path = resolve_binary_path("non_existent_binary_xyz");
assert!(path.ends_with("non_existent_binary_xyz"));
```
