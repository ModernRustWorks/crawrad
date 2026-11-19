---
id: acp-acpconfig-resolve-cli-path
kind: doc
target_file: src/acp.rs
line_hint: 505
author: 
created: 
title: "AcpConfig::resolve_cli_path"
tags: []
signature: "pub fn resolve_cli_path (binary : & str) -> PathBuf"
---

Resolves the absolute path of CLI binaries.

# Examples

```rust
use crawrad_agent::AcpConfig;

let path = AcpConfig::resolve_cli_path("claude");
assert!(!path.as_os_str().is_empty());
```
