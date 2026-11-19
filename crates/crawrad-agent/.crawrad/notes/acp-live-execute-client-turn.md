---
id: acp-live-execute-client-turn
kind: doc
target_file: src/acp_live.rs
line_hint: 391
author: 
created: 
title: "execute_client_turn"
tags: []
signature: "pub fn execute_client_turn (client : & Arc < AcpClient > , prompt : & str , images : & [crate :: chat_attachment :: ImageAttachment] , cwd : Option < & Path > , mcp_servers : Vec < Value > , status : & mut StatusFeed , is_cancelled : impl Fn () -> bool , on_perm_request : impl Fn (Arc < PermissionRequest >) , on_write : impl FnMut (Arc < FileWriteRequest >) ,) -> TurnResult"
---

Executes a prompt turn on an ACP client.

Executes a prompt turn on an ACP client, driving stream collection and returning
the outcome; every event moves `status`, the live status line.

# Examples

```rust,no_run
// Requires spawned ACP client process.
use crawrad_agent::acp_live::execute_client_turn;
use crawrad_agent::acp_status::StatusFeed;
use crawrad_agent::client::AcpClient;
use std::path::Path;
use std::sync::Arc;
use std::sync::mpsc::channel;

let client = Arc::new(AcpClient::spawn(Path::new("dummy"), &[], None, false).unwrap());
let (tx, _rx) = channel();
let mut status = StatusFeed::start("test", false, tx);
let res = execute_client_turn(&client, "hello", &[], None, Vec::new(), &mut status, || false, |_| {}, |_| {});
assert!(res.is_ok());
```
