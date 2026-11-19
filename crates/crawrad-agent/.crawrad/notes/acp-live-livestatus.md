---
id: acp-live-livestatus
kind: doc
target_file: src/acp_live.rs
line_hint: 20
author: 
created: 
title: "LiveStatus"
tags: []
signature: "pub enum LiveStatus"
---

The lifecycle status of the live ACP client for the current conversation.

# Examples

```rust
use crawrad_agent::LiveStatus;

let status = LiveStatus::Idle;
assert_eq!(status, LiveStatus::Idle);
```
