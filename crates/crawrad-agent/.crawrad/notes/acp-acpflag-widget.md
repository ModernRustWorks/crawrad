---
id: acp-acpflag-widget
kind: doc
target_file: src/acp.rs
line_hint: 163
author: 
created: 
title: "AcpFlag::widget"
tags: []
signature: "pub fn widget (self) -> & 'static str"
---

Its widget, by the DSL name.

# Examples

```rust
use crawrad_agent::acp::AcpFlag;

assert_eq!(AcpFlag::AutoApproveReads.widget(), "opt_ai_auto_approve");
```
