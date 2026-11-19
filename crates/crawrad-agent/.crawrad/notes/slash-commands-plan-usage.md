---
id: slash-commands-plan-usage
kind: doc
target_file: src/slash_commands.rs
line_hint: 321
author: 
created: 
title: "plan_usage"
tags: []
signature: "pub fn plan_usage () -> & 'static str"
---

Usage hint for the `/plan` slash command.

# Examples

```rust
use crawrad_agent::slash_commands::plan_usage;

assert!(plan_usage().contains("/plan"));
```
