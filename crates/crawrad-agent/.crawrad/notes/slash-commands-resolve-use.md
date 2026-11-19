---
id: slash-commands-resolve-use
kind: doc
target_file: src/slash_commands.rs
line_hint: 206
author: 
created: 
title: "resolve_use"
tags: []
signature: "pub fn resolve_use (name : & str) -> Option < & 'static str >"
---

Resolves an agent name or id to the provider display name.

Resolves an agent name or id to the provider display name for `AcpConfig::switch_provider`.

# Examples

```rust
use crawrad_agent::slash_commands::resolve_use;

assert!(resolve_use("claude").is_some());
assert!(resolve_use("nonexistent").is_none());
```
