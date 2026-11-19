---
id: client-acpclient-load-session
kind: doc
target_file: src/client.rs
line_hint: 1117
author: 
created: 
title: "AcpClient::load_session"
tags: []
signature: "pub fn load_session (& self , session_id : & str , cwd : Option < & Path > , mcp_servers : Vec < serde_json :: Value > ,) -> Result < String , String >"
---

Loads an existing ACP session with workspace and MCP servers.

Loads an existing ACP session (`session/load`), configuring workspace cwd and local MCP servers.

# Errors

Returns an error if the request fails or communication cannot be established.

# Examples

```rust,no_run
// Loads session from external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
let id = client.load_session("s1", None, vec![]).unwrap();
assert!(!id.is_empty());
```
