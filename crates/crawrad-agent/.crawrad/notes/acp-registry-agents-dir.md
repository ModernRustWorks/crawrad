---
id: acp-registry-agents-dir
kind: doc
target_file: src/acp_registry.rs
line_hint: 103
author: 
created: 
title: "agents_dir"
tags: []
signature: "pub fn agents_dir () -> PathBuf"
---

Returns the user's CrawRAD agents data directory.

# Examples

```rust
use crawrad_agent::agents_dir;

let dir = agents_dir();
assert!(!dir.as_os_str().is_empty());
```
