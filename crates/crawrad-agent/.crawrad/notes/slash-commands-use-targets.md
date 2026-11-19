---
id: slash-commands-use-targets
kind: doc
target_file: src/slash_commands.rs
line_hint: 198
author: 
created: 
title: "use_targets"
tags: []
signature: "pub fn use_targets () -> Vec < (& 'static str , & 'static str) >"
---

Available target agents for switching via `/use`.

Available target agents for switching via `/use`, mapped from registered profiles.

# Examples

```rust
use crawrad_agent::slash_commands::use_targets;

let targets = use_targets();
assert!(!targets.is_empty());
```
