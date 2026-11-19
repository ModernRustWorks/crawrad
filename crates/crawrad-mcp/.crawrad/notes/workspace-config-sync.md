---
id: workspace-config-sync
kind: doc
target_file: src/workspace_config.rs
line_hint: 8
author: 
created: 
title: "sync"
tags: []
signature: "pub fn sync (workspace_root : & std :: path :: Path , instance_id : & str)"
---

Synchronizes the workspace .mcp.json configuration for the bridge.

Writes or updates `.mcp.json` in the workspace root pointing to `crawrad-mcp-bridge.exe`.

# Examples

```rust
use crawrad_mcp::workspace_config::sync;

sync(&std::env::temp_dir(), "inst-123");
```

# Panics

Panics if the internal configuration object cannot be accessed as a mutable object.
