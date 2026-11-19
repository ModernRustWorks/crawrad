---
id: slash-commands-plan-status-label
kind: doc
target_file: src/slash_commands.rs
line_hint: 307
author: 
created: 
title: "plan_status_label"
tags: []
signature: "pub fn plan_status_label (auto_approve : bool) -> & 'static str"
---

Status label for the plan card header.

# Examples

```rust
use crawrad_agent::slash_commands::plan_status_label;

assert_eq!(plan_status_label(true), "🟢 Auto-Approved");
assert_eq!(plan_status_label(false), "Pending Review");
```
