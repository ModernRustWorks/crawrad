---
id: acp-acpflag-all
kind: doc
target_file: src/acp.rs
line_hint: 152
author: 
created: 
title: "AcpFlag::ALL"
tags: []
signature: "pub const ALL : [Self ; 2] = [Self :: AutoApproveReads , Self :: NativeTools]"
---

The two, in the order the page shows them.

# Examples

```rust
use crawrad_agent::acp::AcpFlag;

assert_eq!(AcpFlag::ALL.len(), 2);
```
