---
id: acp-acpconfig-reviews-edits
kind: doc
target_file: src/acp.rs
line_hint: 595
author: 
created: 
title: "AcpConfig::reviews_edits"
tags: []
signature: "pub fn reviews_edits(&self) -> bool"
---

Whether agent edits must wait for the user's review (Wait for approval).

# Examples

```rust
let cfg = crawrad_agent::acp::AcpConfig::default();
assert!(cfg.reviews_edits());
```
