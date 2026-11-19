---
id: acp-acpflag
kind: doc
target_file: src/acp.rs
line_hint: 144
author: 
created: 
title: "AcpFlag"
tags: []
signature: "pub enum AcpFlag"
---

A check box of the Options AI page.

A check box of the Options AI page: what it says, its widget and the
[`AcpConfig`] field it reads and writes.

# Examples

```rust
use crawrad_agent::acp::AcpFlag;

assert_eq!(AcpFlag::ALL.len(), 2);
```
