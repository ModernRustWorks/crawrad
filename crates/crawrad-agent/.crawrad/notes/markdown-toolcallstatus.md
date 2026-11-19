---
id: markdown-toolcallstatus
kind: doc
target_file: src/markdown.rs
line_hint: 18
author: 
created: 
title: "ToolCallStatus"
tags: []
signature: "pub enum ToolCallStatus"
---

Execution state of an agent tool call.

# Examples

```rust
use crawrad_agent::ToolCallStatus;

assert_eq!(ToolCallStatus::Success.badge_label(), "OK");
```
