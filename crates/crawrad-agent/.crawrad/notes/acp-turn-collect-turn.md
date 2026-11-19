---
id: acp-turn-collect-turn
kind: doc
target_file: src/acp_turn.rs
line_hint: 24
author: 
created: 
title: "collect_turn"
tags: []
signature: "pub fn collect_turn (rx : & Receiver < AcpStreamEvent > , mut cancelled : impl FnMut () -> bool , mut on_status : impl FnMut (& AcpStreamEvent) , mut ask : impl FnMut (Arc < PermissionRequest >) , mut on_write : impl FnMut (Arc < FileWriteRequest >) ,) -> TurnEnd"
---

Reads events from rx until the prompt turn finishes.

Reads `rx` until the turn is done. `cancelled` is asked before each event;
`on_status` sees every event before it is folded (the live status line);
a permission request is handed to `ask` (the shell shows the card).

# Examples

```rust
use crawrad_agent::AcpStreamEvent;
use crawrad_agent::acp_turn::{TurnEnd, collect_turn};
use std::sync::mpsc::channel;

let (tx, rx) = channel();
tx.send(AcpStreamEvent::Done { stop_reason: "end_turn".to_string() }).unwrap();
let end = collect_turn(&rx, || false, |_| {}, |_| {}, |_| {});
assert!(matches!(end, TurnEnd::Reply(..)));
```
