---
id: client-acpclient-prompt
kind: doc
target_file: src/client.rs
line_hint: 1239
author: 
created: 
title: "AcpClient::prompt"
tags: []
signature: "pub fn prompt (& self , prompt_text : & str , images : & [crate :: chat_attachment :: ImageAttachment] , cwd : Option < & Path > , mcp_servers : Vec < serde_json :: Value > ,) -> Result < Receiver < AcpStreamEvent > , String >"
---

Sends a prompt turn, streaming updates via the returned receiver.

Sends a prompt turn, streaming updates incrementally via the returned Receiver.

# Errors

Returns an error if ensuring a session fails, locking stdin fails, or sending fails.

# Panics

Panics if the stream channel mutex or pending responses mutex is poisoned.

# Examples

```rust,no_run
// Sends prompt to external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
let rx = client.prompt("hello", &[], None, vec![]).unwrap();
```
