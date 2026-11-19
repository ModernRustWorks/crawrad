---
id: acp-live-warmrx
kind: doc
target_file: src/acp_live.rs
line_hint: 46
author: 
created: 
title: "WarmRx"
tags: []
signature: "pub type WarmRx = Receiver < Result < (Arc < AcpClient > , String) , String > >"
---

What a warm-up thread answers: the connected client and its session id.

# Examples

```rust
use crawrad_agent::acp_live::WarmRx;
use std::sync::mpsc::channel;

let (_tx, rx): (_, WarmRx) = channel();
assert!(rx.try_recv().is_err());
```
