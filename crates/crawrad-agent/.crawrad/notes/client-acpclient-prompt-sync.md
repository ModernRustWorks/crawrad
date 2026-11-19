---
id: client-acpclient-prompt-sync
kind: doc
target_file: src/client.rs
line_hint: 1348
author: 
created: 
title: "AcpClient::prompt_sync"
tags: []
signature: "pub fn prompt_sync (& self , prompt_text : & str , cwd : Option < & Path > , mcp_servers : Vec < serde_json :: Value > ,) -> Result < String , String >"
---

Synchronously runs a prompt turn and collects the concatenated response text.

# Errors

Returns an error if sending the prompt fails or the agent returns an error event.

# Examples

```rust,no_run
// Sends prompt synchronously to external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
let text = client.prompt_sync("hello", None, vec![]).unwrap();
assert!(!text.is_empty());
```
