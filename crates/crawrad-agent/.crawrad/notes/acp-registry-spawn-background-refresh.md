---
id: acp-registry-spawn-background-refresh
kind: doc
target_file: src/acp_registry.rs
line_hint: 534
author: 
created: 
title: "spawn_background_refresh"
tags: []
signature: "pub fn spawn_background_refresh ()"
---

Fetches latest registry JSON from official CDN in the background.

# Examples

```rust,no_run
// Spawns background thread with network HTTP request.
use crawrad_agent::spawn_background_refresh;

spawn_background_refresh();
```
