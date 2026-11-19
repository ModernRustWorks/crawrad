---
id: acp-acpflag-label
kind: doc
target_file: src/acp.rs
line_hint: 155
author: 
created: 
title: "AcpFlag::label"
tags: []
signature: "pub fn label (self) -> & 'static str"
---

What the box says.

# Examples

```rust
use crawrad_agent::acp::AcpFlag;

assert_eq!(AcpFlag::AutoApproveReads.label(), "Approve read operations without asking");
```
